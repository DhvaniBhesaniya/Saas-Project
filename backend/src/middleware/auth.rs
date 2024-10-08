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

    let cookie_token = match req.headers().get("cookie").and_then(|h| h.to_str().ok()) {
        Some(cookies) => {
            let token_cookie = cookies.split('"').find(|s| s.contains("token="));
            match token_cookie {
                Some(cookie) => {
                    let token = cookie.split('=').last().unwrap_or_default();
                    token.to_string()
                }
                None => {
                    "".to_string()
                }
            }
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




// use axum::{
//     extract::Request, http::StatusCode, middleware::Next, response::Response, Json,
// };
// use axum_extra::extract::cookie::CookieJar; // Use this for extracting cookies
// use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
// use serde::Deserialize;
// use serde_json::json;

// use crate::configration;

// #[derive(Debug, Deserialize, Clone)]
// pub struct Claimss {
//     pub id: String,
// }

// pub async fn auth_middleware(
//     jar: CookieJar, // This will automatically extract the cookies from the request
//     mut req: Request,
//     next: Next
// ) -> Response {
//     // Extract the JWT token from the cookies using `axum-extra`
//     let cookie_token = match jar.get("token") {
//         Some(cookie) => cookie.value().to_string(),
//         None => {
//             return Response::builder()
//                 .status(StatusCode::UNAUTHORIZED)
//                 .header("Content-Type", "application/json")
//                 .body(
//                     Json(json!({
//                         "success": false,
//                         "message": "Unauthorized: No Token Provided"
//                     }))
//                     .to_string()
//                     .into(),
//                 )
//                 .unwrap();
//         }
//     };

//     // Read the secret from config
//     let jwt_secret = configration::gett::<String>("jwt_secret");

//     // Verify and decode token
//     let token_data: TokenData<Claimss> = match decode::<Claimss>(
//         &cookie_token,
//         &DecodingKey::from_secret(jwt_secret.as_ref()),
//         &Validation::default(),
//     ) {
//         Ok(data) => data,
//         Err(err) => {
//             eprintln!("Token decode error: {:?}", err); // Log the error for debugging
//             return Response::builder()
//                 .status(StatusCode::UNAUTHORIZED)
//                 .header("Content-Type", "application/json")
//                 .body(
//                     Json(json!({
//                         "success": false,
//                         "message": format!("Invalid token: {}", err),
//                     }))
//                     .to_string()
//                     .into(),
//                 )
//                 .unwrap();
//         }
//     };

//     // Attach the user ID (from claims) to the request extensions
//     req.extensions_mut().insert(token_data.claims);

//     // Proceed to the next middleware or handler
//     let response = next.run(req).await;

//     // Return the response from the next middleware or handler
//     response
// }
