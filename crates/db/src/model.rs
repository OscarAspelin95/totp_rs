use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InsertAppResponse {
    pub id: Uuid,
    pub app: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub app_id: Uuid,
    pub code: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAppResponse {
    pub app_id: Uuid,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAppRequest {
    pub app_name: String,
}
