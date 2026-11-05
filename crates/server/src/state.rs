use db::get_db;
use errors::AppError;
use turso::Database;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        let db = get_db().await?;

        Ok(Self { db })
    }
}
