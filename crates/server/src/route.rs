use crate::{
    AppState,
    custom::{GetAppsRequest, RequiresAuth},
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use db::AuthRequest;
use db::{InsertAppResponse, NewAppRequest, NewAppResponse, get_app, get_apps, insert_app};
use serde::{Deserialize, Serialize};
use totp::get_code;
use tracing::{error, info, instrument};

#[instrument]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "healthy".to_string())
}

#[instrument]
pub async fn authorize(State(state): State<AppState>, Json(req): Json<AuthRequest>) -> Response {
    // db response
    let response = match get_app(req.app_id, &state.db).await {
        Ok(response) => response,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // secret -> code
    let (server_code, seconds_left) = match get_code(response.secret.as_bytes()) {
        Ok((server_code, seconds_left)) => (server_code, seconds_left)
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

#[instrument]
pub async fn new_app(State(state): State<AppState>, Json(req): Json<NewAppRequest>) -> Response {
    let response = match insert_app(req.app_name, &state.db).await {
        Ok(response) => response,
        Err(e) => {
            error!(reason = e.to_string());
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
pub async fn apps(
    State(state): State<AppState>,
    RequiresAuth(req): RequiresAuth<GetAppsRequest>,
) -> Response {
    let response = match get_apps(&state.db).await {
        Ok(response) => response,
        Err(e) => {
            error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (StatusCode::OK, Json(AppsResponse { apps: response })).into_response()
}
