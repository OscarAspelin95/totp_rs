use errors::AppError;
use sqlx::PgPool;

pub async fn get_db() -> Result<PgPool, AppError> {
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = PgPool::connect(&db_url).await?;
    Ok(pool)
}
