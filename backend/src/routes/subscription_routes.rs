use axum::routing::post;
use axum::Router;

use crate::{controllers::subscription_controller::{buy_plan, verify_plan}, middleware::auth::auth_middleware};

pub fn create_subscription_routes() -> Router {
    Router::new()
        .route("/api/subscription/buyplan", post(buy_plan).layer(axum::middleware::from_fn(auth_middleware)))
        .route("/api/subscription/verifyplan", post(verify_plan).layer(axum::middleware::from_fn(auth_middleware)))
}
