use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use chrono::{NaiveDateTime, Duration, Utc};
use rand::rngs::OsRng;
use rand::TryRngCore;
use jwt_simple::prelude::RS256KeyPair;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use sqlx::{Pool, Postgres, types::Uuid};
use sha2::{Digest, Sha256};
use crate::{infra, service::signature};


pub async fn authorize(
    pool: &Pool<Postgres>,
    email: &str,
    password: &str,
    code_challenge: &str
) -> Result<String, ()> {
    let user = match infra::user::find_by_email(pool, email).await {
        Ok(user) => user,
        Err(_) => { return Err(()) }
    };

    if user.pass != password {
        return Err(());
    }

    let authorize_code = generate_authorization_code();

    infra::authorize::register_codes(
        pool,
        &authorize_code,
        &code_challenge,
        &user.id)
        .await
        .map_err(|_| ())?;
    Ok(authorize_code)
}


pub async fn issue_tokens(
    pool: &Pool<Postgres>,
    key_pair: &RS256KeyPair,
    authorization_code: &str,
    code_verifier: &str
) -> Result<(String, String), ()> {
    let user_id = verify_code(pool, authorization_code, code_verifier).await?;

    let token = signature::sign(&key_pair, user_id)
        .unwrap();

    let refresh_token = issue_refresh_token(&pool, &user_id)
        .await
        .unwrap();

    Ok((token, refresh_token))
}


pub async fn refresh(
    pool: &Pool<Postgres>,
    key_pair: &RS256KeyPair,
    refresh_token: &str
) -> Result<(String, String), ()> {
    let token_hash = hash(&refresh_token);
    let (user_id, expires_at) = infra::authorize::delete_refresh_token(&pool, &token_hash)
        .await
        .unwrap();

    if expires_at < Utc::now().naive_utc() {
        return Err(());
    }
    let new_token = signature::sign(&key_pair, user_id)
        .unwrap();

    let new_refresh_token = issue_refresh_token(&pool, &user_id)
        .await
        .unwrap();

    Ok((new_token, new_refresh_token))
}


pub async fn logout(
    pool: &Pool<Postgres>,
    refresh_token: &str
) {
    let token_hash = hash(&refresh_token);
    let _ = infra::authorize::delete_refresh_token(&pool, &token_hash)
        .await
        .map_err(|_| ());
    tracing::info!("logouted user")
}


fn generate_authorization_code() -> String {
    let mut bytes = [0u8; 32];
    OsRng.try_fill_bytes(&mut bytes).unwrap();

    URL_SAFE_NO_PAD.encode(bytes)
}


async fn verify_code(
    pool: &Pool<Postgres>,
    authorization_code: &str,
    code_verifier: &str
) -> Result<Uuid, ()> {
    let (user_id, code_challenge) = infra::authorize::find_by_code(pool, authorization_code)
        .await
        .map_err(|_| ())?;

    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hashed = hasher.finalize();
    let generated_challenge = URL_SAFE_NO_PAD.encode(hashed);

    if generated_challenge != code_challenge {
        return Err(());
    }

    infra::authorize::delete_by_code(pool, authorization_code)
        .await
        .map_err(|_| ())?;

    Ok(user_id)
}

async fn issue_refresh_token(
    pool: &Pool<Postgres>,
    user_id: &Uuid,
) -> Result<String, ()> {
    let refresh_token = generate_authorization_code();
    let refresh_token_hash = hash(&refresh_token);

    let expires_at: NaiveDateTime = Utc::now().naive_utc() + Duration::days(180);
    infra::authorize::register_refresh_token(
        &pool, &refresh_token_hash, &user_id, &expires_at
    )
        .await
        .map_err(|_| ())?;

    Ok(refresh_token)
}


fn hash(password: &str) -> String {
    let salt = SaltString::from_b64("asdfasdfasdfasdfasdfasdfasdf").unwrap();

    let argon2 = Argon2::default();

    let h = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(e) => {
            tracing::info!(message=e.to_string(), "Error");
            panic!("hash");
        }
    };

    h
}

