use crate::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use db::{InsertAppResponse, get_app, get_apps, insert_app};
use serde::{Deserialize, Serialize};
use totp::get_code;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[instrument]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "healthy".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    app_id: Uuid,
    code: u32,
}

#[instrument]
pub async fn authorize(State(state): State<AppState>, Json(req): Json<AuthRequest>) -> Response {
    // db connection
    let conn = match state.db.connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // db response
    let response = match get_app(req.app_id, conn).await {
        Ok(response) => response,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // secret -> code
    let server_code = match get_code(response.secret.as_bytes()) {
        Ok(server_code) => server_code,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    info!("authorizing");
    match req.code == server_code {
        true => {
            info!("code valid");
            StatusCode::OK.into_response()
        }
        false => {
            error!("code invalid");
            StatusCode::NOT_ACCEPTABLE.into_response()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAppResponse {
    app_id: Uuid,
    secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAppRequest {
    app_name: String,
}

#[instrument]
pub async fn new_app(State(state): State<AppState>, Json(req): Json<NewAppRequest>) -> Response {
    info!("DB: {:?}", &state.db);

    let conn = match state.db.connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let response = match insert_app(req.app_name, conn).await {
        Ok(response) => response,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (
        StatusCode::OK,
        Json(NewAppResponse {
            app_id: response.id,
            secret: response.secret,
        }),
    )
        .into_response()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppsResponse {
    apps: Vec<InsertAppResponse>,
}

#[instrument]
pub async fn apps(State(state): State<AppState>) -> Response {
    let conn = match state.db.connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let response = match get_apps(conn).await {
        Ok(response) => response,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (StatusCode::OK, Json(AppsResponse { apps: response })).into_response()
}
