// pub fn generate_token_and_set_cookie(user_id: ObjectId, jar: CookieJar) -> CookieJar {
//     let claims = Claims {
//         id: user_id.to_hex(), // Convert ObjectId to hex string
//         exp: 172800,          // Token expiration in seconds (2 days in this example),
//     };
//         let jwt_secret = configration::gett::<String>("jwt_secret");

//     let token = encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(jwt_secret.as_ref()),
//     )
//     .unwrap();

//     // Build the cookie (like in Node.js example)
//     let cookie = Cookie::build("token", token)
//         .http_only(true) // Prevent XSS
//         .secure(true) // Use secure flag for HTTPS
//         .same_site(SameSite::Strict) // Prevent CSRF
//         .max_age(OtherDuration::days(1)) // Expire in 1 day
//         .path("/") // Set cookie for the entire domain
//         .finish();

//     // Add the cookie to the jar (this is equivalent to setting the cookie in the response)
//     jar.add(cookie)
// }

// // Generate token and set the cookie
// let jar = generate_token_and_set_cookie(user_id, jar);

use bson::oid::ObjectId;
use chrono::{Duration, Utc};
use serde::Serialize;

use jsonwebtoken::{encode, EncodingKey, Header};

use axum::http::{HeaderMap, HeaderValue};
// use cookie::{time::Duration as OtherDuration, Cookie};
use cookie::{time::Duration as OtherDuration, Cookie, SameSite};

use crate::configration;

#[derive(Debug, Serialize)]
pub struct Claims {
    id: String,
    exp: i64,
}

pub fn generate_token_and_set_cookie(user_id: ObjectId) -> HeaderMap {
    let jwt_secret = configration::gett::<String>("jwt_secret");

    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(3600 * 24)) // 24 hours
        .expect("valid timestamp")
        .timestamp();
    // Creating JWT token with the user_id in claims
    let claims = Claims {
        id: user_id.to_hex(), // Convert ObjectId to hex string
        exp: expiration_time,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    // httpOnly: true,  // prevent XSS atacks cross-site scripting attacks
    // maxAge: 15 * 24 * 60 * 60 * 1000,  // MS
    // sameSite: "strict", // CSRF attacks cross-site request forgery attacks
    // secure: process.env.NODE_ENV !== "development",

    // // Set the token in a cookie
    let cookie: Cookie = Cookie::build(("token", token))
        // .domain("www.rust-lang.org")
        .path("/")
        .same_site(SameSite::Strict) // Prevent CSRF   // for now SameSite::Strict is not used.
        .secure(false) // for now it is false
        .http_only(true)
        .max_age(OtherDuration::days(1))
        .build();

    // Prepare the headers with Set-Cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        "Set-Cookie",
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    headers
}



pub fn generate_token_and_unset_cookie(user_id: ObjectId) -> HeaderMap {
    let jwt_secret = configration::gett::<String>("jwt_secret");

    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(3600 * 0)) // 0 hours
        .expect("valid timestamp")
        .timestamp();
    // Creating JWT token with the user_id in claims
    let claims = Claims {
        id: user_id.to_hex(), // Convert ObjectId to hex string
        exp: expiration_time,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    // // Set the token in a cookie
    let cookie: Cookie = Cookie::build(("token", token))
        // .domain("www.rust-lang.org")
        .path("/")
        .same_site(SameSite::Strict) // Prevent CSRF   // for now SameSite::Strict is not used.
        .secure(false) // for now it is false
        .http_only(true)
        .max_age(OtherDuration::seconds(0))
        .build();

    // Prepare the headers with Set-Cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        "Set-Cookie",
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    headers
}
