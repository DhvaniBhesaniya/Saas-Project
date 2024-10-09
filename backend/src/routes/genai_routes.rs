use axum::routing::post;
use axum::Router;

use crate::controllers::genai_controller::{genaichat, genaidoc, genaitext};

pub fn create_genai_routes() -> Router {
    Router::new()
        .route("/api/genai/text", post(genaitext))
        .route("/api/genai/doc", post(genaidoc))
        .route("/api/genai/chat", post(genaichat))
}

// altnerate of genai crate is [ https://github.com/avastmick/google-generative-ai-rs ]




