mod route;
use route::{apps, authorize, health, new_app};

mod state;
use state::AppState;

mod custom;

use axum::{
    Router,
    routing::{get, post},
};
use dotenv;
use errors::AppError;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{Level, info};
use tracing_subscriber::{self, EnvFilter};

fn setup_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

async fn run_server() -> Result<(), AppError> {
    let app_state = AppState::new().await?;

    let router: Router<_> = Router::new()
        .route("/", get(health))
        .route("/auth", post(authorize))
        .route("/new_app", post(new_app))
        .route("/apps", get(apps))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    let port: String = std::env::var("PORT").unwrap_or("8080".to_string());
    let addr: String = std::env::var("ADDR").unwrap_or("127.0.0.1".to_string());

    info!(addr = &addr, port = &port, "Starting server");
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr, port)).await?;

    let _ = axum::serve(listener, router).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();
    setup_logging();

    run_server().await?;
    Ok(())
}
