use errors::AppError;
use serde::{Deserialize, Serialize};
use turso::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertAppResponse {
    pub id: Uuid,
    pub app: String,
    pub secret: String,
}

impl TryFrom<Row> for InsertAppResponse {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let parsed = Self {
            id: Uuid::parse_str(&row.get::<String>(0)?).unwrap(),
            app: row.get::<String>(1)?,
            secret: row.get::<String>(2)?,
        };

        Ok(parsed)
    }
}
