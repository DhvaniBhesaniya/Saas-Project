use crate::controllers::user_controller::{
    get_user_data, login_user, logout_user, register_user, update_user_data
};
use crate::middleware::auth::auth_middleware;
use axum::extract::DefaultBodyLimit;

use axum::{
    routing::{get, post},
    Router,
};


pub fn create_user_routes() -> Router {
    Router::new()
        .route("/api/user/register", post(register_user))
        .route("/api/user/login", post(login_user))
        .route("/api/user/logout", post(logout_user).layer(axum::middleware::from_fn(auth_middleware)))
        .route(
            "/api/user/userdata",
            get(get_user_data).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route(
            "/api/user/updateuser",
            post(update_user_data).layer(DefaultBodyLimit::max(10 * 1024 * 1024)).layer(axum::middleware::from_fn(auth_middleware)))  // 10 mb 
        
}
