use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    log::info!("cors layer builted.");

    CorsLayer::new()
        // Only allow the origin you want (localhost:5500)
        .allow_origin("http://localhost:5170".parse::<HeaderValue>().unwrap())
        // Allow these HTTP methods (GET, POST, OPTIONS)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        // Allow certain headers if needed (e.g., Authorization, Content-Type)
        .allow_headers(Any)
}

// use axum::http::{HeaderValue, Method};
// use tower_http::cors::{Any, CorsLayer};

// pub fn cors_layer() -> CorsLayer {
//     log::info!("inside cors layer");

//     let origins = ["http://localhost:5500".parse().unwrap(),
//                     http://localhost:5500".parse().unwrap()];
//     CorsLayer::new()
//         .allow_origin(origins)
//         .allow_methods([Method::GET, Method::POST])
// }
