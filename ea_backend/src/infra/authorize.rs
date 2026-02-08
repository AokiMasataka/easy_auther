use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres, types::Uuid};


pub async fn register_codes(
    pool: &Pool<Postgres>,
    authorization_code: &str,
    code_challenge: &str,
    user_id: &Uuid
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO authorize_codes
            (authorize_code, code_challenge, user_id)
        VALUES
            ($1, $2, $3)
        "#,
        authorization_code,
        code_challenge,
        user_id
    )
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn find_by_code(
    pool: &Pool<Postgres>,
    authorization_code: &str,
) -> Result<(Uuid, String), sqlx::Error> {
    let record = sqlx::query!(
        r#"
        SELECT user_id, code_challenge
        FROM authorize_codes
        WHERE authorize_code = $1
        "#,
        authorization_code
    )
        .fetch_one(pool)
        .await?;

    Ok((record.user_id, record.code_challenge))
}


pub async fn delete_by_code(
    pool: &Pool<Postgres>,
    authorization_code: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM authorize_codes
        WHERE authorize_code = $1
        "#,
        authorization_code
    )
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn register_refresh_token(
    pool: &Pool<Postgres>,
    token_hash: &str,
    user_id: &Uuid,
    expiers_at: &NaiveDateTime
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO refresh_tokens
            (token_hash, user_id, expires_at)
        VALUES
            ($1, $2, $3)
        "#,
        token_hash,
        user_id,
        expiers_at
    )
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn delete_refresh_token(
    pool: &Pool<Postgres>,
    token_hash: &str
) -> Result<(Uuid, NaiveDateTime), sqlx::Error> {
    let deleted = sqlx::query!(
        r#"
        DELETE FROM
            refresh_tokens
        WHERE
            token_hash = $1
        RETURNING
            user_id, expires_at
        "#,
        token_hash,
    )
        .fetch_one(pool)
        .await?;
    Ok((deleted.user_id, deleted.expires_at))
}
