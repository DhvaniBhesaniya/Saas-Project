use axum::{routing::get, Router};
use middleware::cors::cors_layer;
use routes::genai_routes::create_genai_routes;
use routes::subscription_routes::create_subscription_routes;
use tower::ServiceBuilder;
use utils::db::connect_db;
use utils::set_env::set_env_variable;

use crate::routes::user_route::create_user_routes;
use std::net::SocketAddr;

use tokio;

mod configration;
mod controllers;
mod middleware;
mod models;
mod routes;
mod utils;

use crate::configration::gett;
use crate::utils::logger;
#[tokio::main]
async fn main() {
    // Initialize Logger
    logger::startLogger();
    let _db_client = connect_db().await;
    let _set_env = set_env_variable().await;

    let app = Router::new()
        .route("/test", get(handler))
        .merge(create_user_routes())
        .merge(create_genai_routes())
        .merge(create_subscription_routes())
        .layer(ServiceBuilder::new().layer(cors_layer()));

    let addr = SocketAddr::from(([127, 0, 0, 1], gett("port")));

    println!("Server running on http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    log::info!("Rust API working");
    "Saas API working.."
}
