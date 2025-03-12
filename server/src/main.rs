mod crud;
mod db_controller;
mod handlers;
mod middlewares;
mod models;
mod utils;

use std::sync::Arc;

use anyhow::Result;
use appledb_common::{
    config::{ListenMode, read_configuration},
    routes::{ADMIN_ROUTES, PublicRoutes},
};
use axum::{Router, extract::DefaultBodyLimit};
use clap::Parser;
use db_controller::DBController;
use middlewares::log_requests;
use models::{AppState, Opts};
use tokio::net::{TcpListener, UnixListener};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    let configuration = read_configuration(opts.config_path).await?;

    env_logger::init();

    let db_controller = DBController::new(configuration.database_path).await?;

    let state = AppState { db_controller };

    let app = Router::new()
        .nest(ADMIN_ROUTES, handlers::get_admin_router())
        .nest(PublicRoutes::route_prefix(), handlers::get_public_router())
        .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(log_requests)))
        .layer(DefaultBodyLimit::max(configuration.http_max_body_size))
        .with_state(Arc::new(state));

    log::info!("Server listening on {}...", configuration.listen_mode);
    match configuration.listen_mode {
        ListenMode::SocketAddr(socket_addr) => Ok(axum::serve(
            TcpListener::bind(socket_addr).await?,
            app.into_make_service(),
        )
        .await?),
        ListenMode::UnixSocket(path) => {
            Ok(axum::serve(UnixListener::bind(path)?, app.into_make_service()).await?)
        }
    }
}
