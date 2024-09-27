use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::Deserialize;
use serde_json::json;

use crate::configration;

#[derive(Debug, Deserialize, Clone)]
pub struct Claimss {
    pub id: String,
}

pub async fn auth_middleware(mut req: Request, next: Next) -> Response {
    // Extract token from the request
    let cookie_token = match req.headers().get("token").and_then(|h| h.to_str().ok()) {
        Some(cookies) => {
            // dbg!(cookies);
            cookies
        }
        None => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(
                    Json(json!({
                        "success": false,
                        "message": "Authorization token not found in cookies"
                    }))
                    .to_string()
                    .into(),
                )
                .unwrap();
        }
    };

    // Read the secret from config
    let jwt_secret = configration::gett::<String>("jwt_secret");

    // Verify and decode token
    let token_data: TokenData<Claimss> = match decode::<Claimss>(
        &cookie_token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Token decode error: {:?}", err); // Log the error for debugging
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(
                    Json(json!({
                        "success": false,
                        "message": format!("Invalid token: {}", err),
                    }))
                    .to_string()
                    .into(),
                )
                .unwrap();
        }
    };

    // Attach the user ID (from claims) to the request extensions
    req.extensions_mut().insert(token_data.claims);

    // Proceed to the next middleware or handler
    let response = next.run(req).await;

    // Return the response from the next middleware or handler
    response
}
