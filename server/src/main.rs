mod crud;
mod db_controller;
mod handlers;
mod middlewares;
mod models;
mod utils;

use anyhow::{Context, Result};
use appledb_common::{
    config::{ListenMode, read_configuration},
    routes::{ADMIN_ROUTES, PublicRoutes},
};
use axum::{
    Router,
    body::Body,
    extract::{DefaultBodyLimit, State},
    http::{Method, Request, StatusCode},
    response::IntoResponse,
};
use clap::Parser;
use db_controller::DBController;
use middlewares::log_requests;
use models::{AppState, Opts};
use std::sync::LazyLock;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    net::{TcpListener, UnixListener},
    sync::RwLock,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

// Models coming from https://gist.github.com/adamawolf/3048717
pub static APPLE_MODELS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../apple_models.json"))
        .expect("cannot deserialize apple models")
});

async fn handle_webapp(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
) -> impl IntoResponse {
    let mut serve_dir =
        ServeDir::new(&state.web_sources_path).append_index_html_on_directories(true);

    match serve_dir.try_call(request).await {
        Ok(response) if response.status() != StatusCode::NOT_FOUND => {
            response.map(axum::body::Body::new)
        }
        _ => {
            // Return 'index.html' for all requests to let client-side routing do the job
            let index_path = state.web_sources_path.join("index.html");
            match ServeFile::new(index_path)
                .try_call(Request::new(Body::empty()))
                .await
            {
                Ok(response) => response.map(axum::body::Body::new),
                Err(e) => {
                    log::error!("Error while serving file: {e}");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let configuration = read_configuration(opts.config_path).await?;

    utils::setup_logger();

    let db_controller = DBController::new(configuration.database_url).await?;
    let state = Arc::new(AppState {
        db_controller: Arc::new(db_controller),
        web_sources_path: configuration.web_sources_path,
        max_concurrent_tasks: configuration.max_concurrent_tasks,
        running_entitlements_tasks: Arc::new(RwLock::new(HashMap::new())),
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .nest(ADMIN_ROUTES, handlers::get_admin_router())
        .nest(PublicRoutes::route_prefix(), handlers::get_public_router())
        .fallback(handle_webapp)
        .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(log_requests)))
        .layer(DefaultBodyLimit::max(configuration.http_max_body_size))
        .layer(cors)
        .with_state(state);

    log::info!("Server listening on {}...", configuration.listen_mode);
    match configuration.listen_mode {
        ListenMode::SocketAddr(socket_addr) => Ok(axum::serve(
            TcpListener::bind(socket_addr).await?,
            app.into_make_service(),
        )
        .await?),
        ListenMode::UnixSocket(path) => {
            if path.try_exists()? {
                log::info!("Removing old unix socket...");
                std::fs::remove_file(&path)
                    .with_context(|| format!("cannot delete unix socket at path {:?}", path))?;
            }

            Ok(axum::serve(UnixListener::bind(path)?, app.into_make_service()).await?)
        }
    }
}
