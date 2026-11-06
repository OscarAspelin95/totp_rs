use axum::{
    Json,
    extract::{FromRequest, Request},
    http::{StatusCode, header::AUTHORIZATION},
};
use serde::{Deserialize, de::DeserializeOwned};
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct GetAppsRequest;

pub struct RequiresAuth<T>(pub T);

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

        match bearer_token {
            None => return Err((StatusCode::UNAUTHORIZED, "expected a bearer token")),
            Some(token) if token.is_empty() => {
                return Err((StatusCode::UNAUTHORIZED, "invalid bearer token"));
            }
            _ => {}
        }

        let Json(body) = Json::<T>::from_request(req, state).await.map_err(|e| {
            error!("{e}");
            (StatusCode::BAD_REQUEST, "invalid json body")
        })?;

        Ok(RequiresAuth(body))
    }
}
