use axum::{
    Json,
    extract::{FromRequest, Request},
    http::{StatusCode, header::AUTHORIZATION},
};
use serde::{Deserialize, de::DeserializeOwned};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct GetAppsRequest;

pub struct RequiresAuth<T>(pub T);

/// General auth extractor for generic request `T`.
impl<S, T> FromRequest<S> for RequiresAuth<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Pretend any non-empty bearer token is valid.
        let bearer_token = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        let admin_pwd = match std::env::var("ADMIN_PASSWORD") {
            Ok(admin_pwd) => admin_pwd,
            Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "admin password not set.")),
        };

        match bearer_token {
            None => return Err((StatusCode::UNAUTHORIZED, "expected a bearer token")),
            Some(token) if token == &admin_pwd => {
                info!("token valid");
            }
            _ => return Err((StatusCode::UNAUTHORIZED, "invalid token")),
        }

        // TO extract query, probably use FromRequestParts.
        let Json(body) = Json::<T>::from_request(req, state).await.map_err(|e| {
            error!("{e}");
            (StatusCode::BAD_REQUEST, "invalid json body")
        })?;

        Ok(RequiresAuth(body))
    }
}
