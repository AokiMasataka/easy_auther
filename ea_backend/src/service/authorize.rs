use rand::rngs::OsRng;
use rand::TryRngCore;
use jwt_simple::prelude::RS256KeyPair;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use sqlx::{Pool, Postgres, types::Uuid};
use sha2::{Digest, Sha256};
use crate::{infra, service::signature};




fn generate_authorization_code() -> String {
    let mut bytes = [0u8; 32];
    OsRng.try_fill_bytes(&mut bytes).unwrap();

    // URL-safe & paddingなし
    URL_SAFE_NO_PAD.encode(bytes)
}


pub async fn authorize(
    pool: &Pool<Postgres>,
    email: &str,
    password: &str,
    code_challenge: &str
) -> Result<String, ()> {
    
    // email & pass
    let user = match infra::user::find_by_email(pool, email).await {
        Ok(user) => user,
        Err(_) => {
            // handle error
            return Err(());
        }
    };

    if user.pass != password {
        // handle invalid password
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
    // register_codes
    Ok(authorize_code)
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
        // handle invalid code_verifier
        return Err(());
    }

    infra::authorize::delete_by_code(pool, authorization_code)
        .await
        .map_err(|_| ())?;

    Ok(user_id)
}


pub async fn token(
    pool: &Pool<Postgres>,
    key_pair: &RS256KeyPair,
    authorization_code: &str,
    code_verifier: &str
) -> Result<String, ()> {
    let user_id = verify_code(pool, authorization_code, code_verifier).await?;

    let user = infra::user::find_by_id(pool, user_id)
        .await
        .map_err(|_| ())?;

    Ok(
        signature::sign(
        &key_pair,
        user_id,
        &user.name,
        false
        )
    )
}
