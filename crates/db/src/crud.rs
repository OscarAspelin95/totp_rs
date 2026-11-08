use errors::AppError;
use sqlx::{PgPool, query_as};
use uuid::Uuid;

use crate::model::InsertAppResponse;
use totp::generate_random_secret;

pub async fn insert_app(app_name: String, conn: &PgPool) -> Result<InsertAppResponse, AppError> {
    let secret = generate_random_secret();

    let response = query_as!(
        InsertAppResponse,
        r#"
        INSERT INTO apps
            (app, secret)
        VALUES
            ($1, $2)
        RETURNING
            id, app, secret
        "#,
        app_name,
        secret
    )
    .fetch_one(conn)
    .await?;

    Ok(response)
}

pub async fn get_app(app_id: Uuid, conn: &PgPool) -> Result<InsertAppResponse, AppError> {
    let response = query_as!(
        InsertAppResponse,
        r#"
        SELECT
            id, app, secret
        FROM
            apps
        WHERE
            id = $1;
        "#,
        app_id
    )
    .fetch_one(conn)
    .await?;

    Ok(response)
}

pub async fn get_apps(conn: &PgPool) -> Result<Vec<InsertAppResponse>, AppError> {
    let response = query_as!(
        InsertAppResponse,
        r#"
        SELECT
            id, app, secret
        FROM
            apps;
        "#
    )
    .fetch_all(conn)
    .await?;

    Ok(response)
}
