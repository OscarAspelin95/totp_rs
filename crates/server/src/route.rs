use crate::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use totp::generate_random_secret;
use totp::get_code;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[instrument]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "healthy".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    user_id: Uuid,
    code: u32,
}

#[instrument]
pub async fn authorize(
    State(state): State<AppState>,
    Json(req): Json<AuthRequest>,
) -> impl IntoResponse {
    info!("DB: {:?}", &state.db);

    {}
    // Check if user has secret
    let db = state.db.read().await;

    let secret = match db.get(&req.user_id.to_string()) {
        Some(secret) => secret.as_bytes(),
        None => return (StatusCode::NOT_FOUND, "user not found"),
    };

    let server_code = match get_code(secret) {
        Ok(server_code) => server_code,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, ""),
    };

    info!("authorizing");
    match req.code == server_code {
        true => {
            info!("code valid");
            (StatusCode::OK, "code valid")
        }
        false => {
            error!("code invalid");
            (StatusCode::NOT_ACCEPTABLE, "code invalid")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserResponse {
    user_id: Uuid,
    secret: String,
}

#[instrument]
pub async fn new_user(State(state): State<AppState>) -> Json<NewUserResponse> {
    let user_id = Uuid::now_v7();
    let secret = generate_random_secret();

    // unlock directly after insert.
    {
        let mut db = state.db.write().await;
        let _ = db.insert(user_id.to_string().clone(), secret.clone());
    }

    info!("DB: {:?}", &state.db);

    Json(NewUserResponse { user_id, secret })
}
