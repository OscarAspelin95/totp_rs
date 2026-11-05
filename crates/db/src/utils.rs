use errors::AppError;
use std::path::PathBuf;
use turso::{Builder, Database};

pub fn ensure_path(p: String) -> Result<String, AppError> {
    let path = PathBuf::from(&p);

    match path.is_file() {
        true => Ok(p),
        false => Err(AppError::FileDoesNotExistError(p)),
    }
}

pub fn get_db_path() -> Result<String, AppError> {
    let db_path = std::env::var("DB_PATH")?;
    return ensure_path(db_path);
}

pub async fn get_db() -> Result<Database, AppError> {
    let db_path = get_db_path()?;
    let db = Builder::new_local(db_path.as_str()).build().await?;

    Ok(db)
}
