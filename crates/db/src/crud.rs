use errors::AppError;
use turso::Connection;
use uuid::Uuid;

use crate::model::InsertAppResponse;
use totp::generate_random_secret;

pub async fn create_table(conn: &Connection) -> Result<(), AppError> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS apps (
            id TEXT PRIMARY KEY DEFAULT (uuid7_str()),
            app TEXT UNIQUE COLLATE NOCASE NOT NULL CHECK(length(app) >= 3),
            secret TEXT NOT NULL CHECK(length(secret) = 32)
        );
        "#,
        (),
    )
    .await?;

    Ok(())
}

pub async fn insert_app(app_name: String, conn: Connection) -> Result<InsertAppResponse, AppError> {
    let mut rows = conn
        .query(
            r#"
        INSERT INTO apps
            (app, secret)
        VALUES
            (?1, ?2)
        RETURNING id, app, secret;
        "#,
            (app_name, generate_random_secret()),
        )
        .await?;

    let row = rows.next().await?.ok_or_else(|| AppError::Base32Error)?;
    let response: InsertAppResponse = row.try_into()?;

    Ok(response)
}

pub async fn get_app(app_id: Uuid, conn: Connection) -> Result<InsertAppResponse, AppError> {
    let mut rows = conn
        .query(
            r#"
        SELECT id, app, secret
            FROM apps
        WHERE id = (?1);
        "#,
            (app_id.to_string(),),
        )
        .await?;

    let row = rows.next().await?.ok_or_else(|| AppError::Base32Error)?;
    let response: InsertAppResponse = row.try_into()?;

    Ok(response)
}

pub async fn get_apps(conn: Connection) -> Result<Vec<InsertAppResponse>, AppError> {
    let mut rows = conn
        .query(
            r#"
        SELECT id, app, secret
            FROM apps;
        "#,
            (),
        )
        .await?;

    let mut response = vec![];

    while let Some(row) = rows.next().await? {
        response.push(row.try_into()?);
    }

    Ok(response)
}
