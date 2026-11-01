use dotenv;
use errors::AppError;
use std::path::PathBuf;
use tokio;
use totp::generate_random_secret;
use turso::{Builder, Connection};
use uuid::Uuid;

fn ensure_path(p: String) -> Result<String, AppError> {
    let path = PathBuf::from(&p);

    match path.is_file() {
        true => Ok(p),
        false => Err(AppError::FileDoesNotExistError(p)),
    }
}

fn get_db() -> Result<String, AppError> {
    let db_path = std::env::var("DB_PATH")?;
    return ensure_path(db_path);
}

async fn create_table(conn: &Connection) -> Result<(), AppError> {
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

#[derive(Debug)]
struct InsertAppResponse {
    id: Uuid,
    app: String,
    secret: String,
}

async fn insert_app(app_name: String, conn: &Connection) -> Result<InsertAppResponse, AppError> {
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

    let response = InsertAppResponse {
        id: Uuid::parse_str(&row.get::<String>(0)?).unwrap(),
        app: row.get::<String>(1)?,
        secret: row.get::<String>(2)?,
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    let db_path = get_db()?;

    let db = Builder::new_local(db_path.as_str()).build().await?;

    let conn = db.connect()?;

    let _ = create_table(&conn).await?;

    let response = insert_app("MyApp".to_string(), &conn).await?;

    println!("{:?}", response);

    Ok(())
}
