use crate::models::user_model::{SubscriptionPlan, Usage, User};
use crate::utils::generate_token::{generate_token_and_set_cookie, generate_token_and_unset_cookie};
use crate::middleware::auth::Claimss;
use axum::response::Response;
use axum::{
    extract::Form,
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use bson::Document;
// use chrono::{Duration, Utc};

// use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::{doc, oid::ObjectId, Bson, DateTime as BsonDateTime};

use serde::{Deserialize, Serialize};
use serde_json::json;

use validator::validate_email;

// use axum::http::{HeaderMap, HeaderValue};
// // use cookie::{time::Duration as OtherDuration, Cookie};
// use cookie::{Cookie, SameSite};

// use axum_extra::extract::cookie::CookieJar;
// use cookie::time::Duration as OtherDuration; // Use from axum_extra

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct Claims {
    id: String,
    exp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
    pub profile_img: Option<String>,
    pub tries_used: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    #[serde(rename = "_id")]
    pub id: String, // User's ID
    pub name: String,                        // User's name
    pub email: String,                       // User's email
    pub google_id: Option<String>,           // Optional google_id
    pub login_type: Option<String>,          // Optional login type
    pub profile_img: Option<String>,         // Optional profile image
    pub subscription_plan: SubscriptionPlan, // User's subscription plan details
    pub usage: Usage,                        // User's usage data
}

pub async fn register_user(Json(payload): Json<RegisterUser>) -> Response {
    let collection = User::get_user_collection().await;

    // Checking if the user already exists
    if collection
        .find_one(doc! { "email": &payload.email })
        .await
        .unwrap()
        .is_some()
    {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(
                Json(json!({ "success": false, "message": "User already exists" }))
                    .to_string()
                    .into(),
            )
            .unwrap();
    }

    // Validating email format & strong password
    if !validate_email(&payload.email) {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(
                Json(json!({ "success": false, "message": "Please enter a valid email." }))
                    .to_string()
                    .into(),
            )
            .unwrap();
    }
    if payload.password.len() < 8 {
        return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Json(json!({ "success": false, "message": "Password must be at least 8 characters long." })).to_string().into())
        .unwrap();
    }

    // Hashing the user's password
    let hashed_password = hash(&payload.password, DEFAULT_COST).unwrap();

    // Creating the username, from the name of the user.
    let username = payload.name.to_uppercase();

    // Step 6: Construct the new user document
    let new_user_doc = doc! {
        "name": &payload.name,
        "email": &payload.email,
        "username": username,
        "password": &hashed_password,
        "google_id": None::<Bson>, // Not a Google login
        "login_type": "email", // Email login
        "profileImg": None::<Bson>, // Optional field
        "subscription_plan": {
            "plan_type": "free", // Default to "free"
            "start_date":BsonDateTime::now().to_string(),
            "end_date": None::<Bson>, // None as optional date
            "payment_status": None::<Bson>
        },
        "usage": {
            "tries_used": 0, // Start with 0 tries used
            "max_tries": 10,  // Free plan allows 10 tries
        },
        "AccDeleted": false,
        "created_at": BsonDateTime::now().to_string(),
        "updated_at": BsonDateTime::now().to_string()
    };

    // Inserting the new user into the database
    let insert_result = collection.insert_one(new_user_doc.clone()).await.unwrap();

    // Extract the inserted `_id` from MongoDB insert result
    let user_id = insert_result
        .inserted_id
        .as_object_id()
        .expect("Failed to get user _id");

    let headers = generate_token_and_set_cookie(user_id);

    // Construct and return the response
    Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", headers["Set-Cookie"].clone())
        .header("Content-Type", "application/json")
        .body(
            Json(json!({
                "success": true,
                "message": "User registered successfully"
            }))
            .to_string()
            .into(),
        )
        .unwrap()
}

pub async fn login_user(Json(payload): Json<LoginUser>) -> Response {
    let collection = User::get_user_collection().await;

    // Checking if the user exists
    let user_doc = match collection
        .find_one(doc! { "email": &payload.email })
        .await
        .unwrap()
    {
        Some(doc) => doc,
        None => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(
                    Json(json!({ "success": false, "message": "User does not exist" }))
                        .to_string()
                        .into(),
                )
                .unwrap();
        }
    };

    // Verifying password
    let stored_password = user_doc.get_str("password").unwrap();
    if !verify(&payload.password, stored_password).unwrap() {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(
                Json(json!({ "success": false, "message": "Invalid Password" }))
                    .to_string()
                    .into(),
            )
            .unwrap();
    }

    // Extract the user ID from the document
    let user_id = user_doc.get_object_id("_id").unwrap();

    let headers = generate_token_and_set_cookie(user_id);

    // Construct and return the response
    Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", headers["Set-Cookie"].clone())
        .header("Content-Type", "application/json")
        .body(
            Json(json!({
                "success": true,
                "message": "Login successful"
            }))
            .to_string()
            .into(),
        )
        .unwrap()
}


pub async fn logout_user(Extension(claims): Extension<Claimss>)-> Response{
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(oid) => oid,
        Err(_) => {
            return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(
                Json(json!({ "success": false, "message": "Unauthorized  in logout" }))
                    .to_string()
                    .into(),
            )
            .unwrap();
        }
    };


    let headers = generate_token_and_unset_cookie(user_id);

    // Construct and return the response
    Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", headers["Set-Cookie"].clone())
        .header("Content-Type", "application/json")
        .body(
            Json(json!({
                "success": true,
                "message": "Logout successful"
            }))
            .to_string()
            .into(),
        )
        .unwrap()


}

pub async fn get_user_data(Extension(claims): Extension<Claimss>) -> impl IntoResponse {
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(oid) => oid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Invalid user ID" })),
            );
        }
    };

    let collection = User::get_user_collection().await;
    match collection.find_one(doc! { "_id": user_id }).await {
        Ok(Some(mut user_doc)) => {
            if let Some(id) = user_doc.remove("_id") {
                if let Some(object_id) = id.as_object_id() {
                    user_doc.insert("_id", object_id.to_string());
                }
            }

            // Remove the "password" field from the user document
            user_doc.remove("password");

            // Return the user data
            (
                StatusCode::OK,
                Json(json!({ "success": true, "data": user_doc })),
            )
        }
        Ok(None) => {
            // User not found
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "message": "User not found" })),
            )
        }
        Err(e) => {
            // Error during database lookup
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": e.to_string() })),
            )
        }
    }
}

pub async fn update_user_data(
    Extension(claims): Extension<Claimss>,
    Form(form): Form<UpdateUserData>,
) -> impl IntoResponse {
    // Initialize the user collection
    let collection = User::get_user_collection().await;
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Invalid user ID" })),
            )
        }
    };

    // Get the user's document
    let filter = doc! { "_id": user_id };
    let user_doc = match collection.find_one(filter.clone()).await {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "message": "User not found" })),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "Failed to fetch user data" })),
            )
        }
    };

    // Create a document for the updates
    let mut update_fields = Document::new();

    // Update name if provided
    if let Some(name) = &form.name {
        update_fields.insert("name", name.clone());
    }

    // Update email if provided
    if let Some(email) = &form.email {
        update_fields.insert("email", email.clone());
    }

    // Update profile image if provided
    if let Some(profile_img) = &form.profile_img {
        update_fields.insert("profileImg", profile_img.clone());
    }

    // Handle password change
    if let (Some(current), Some(new)) = (&form.current_password, &form.new_password) {
        if current.is_empty() || new.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(
                    json!({ "success": false, "message": "Current password and new password cannot be empty" }),
                ),
            );
        }
        let stored_password = user_doc.get_str("password").unwrap();
        if verify(&current, stored_password).unwrap() {
            let hashed_password = hash(new, DEFAULT_COST).unwrap();
            update_fields.insert("password", hashed_password);
        } else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Current password is incorrect" })),
            );
        }
    }
    // Update the user document in MongoDB
    if !update_fields.is_empty() {
        let update_doc = doc! { "$set": update_fields };
        if collection.update_one(filter, update_doc).await.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "Failed to update user data" })),
            );
        }
    }

    // Return a success response
    (
        StatusCode::OK,
        Json(json!({ "success": true, "message": "User details updated successfully" })),
    )
}

// pub async fn get_user_data(Extension(claims): Extension<Claimss>) -> impl IntoResponse {
//     let user_id = match ObjectId::parse_str(&claims.id) {
//         Ok(oid) => oid,
//         Err(_) => {
//             return (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({ "success": false, "message": "Invalid user ID" })),
//             );
//         }
//     };

//     let collection = User::get_user_collection().await;
//     match collection.find_one(doc! { "_id": user_id }).await {
//         Ok(Some(mut user_doc)) => {
//             if let Some(id) = user_doc.remove("_id") {
//                 if let Some(object_id) = id.as_object_id() {
//                     user_doc.insert("_id", object_id.to_string());
//                 }
//             }

//             // Remove the "password" field from the user document
//             user_doc.remove("password");

//             // Extract the user's data into the UserResponse structure

//             // Handle subscription_plan (nested object)
//             // let subscription_plan_doc = user_doc.get_document("subscription_plan").ok();
//             // let subscription_plan = if let Some(sub_plan) = subscription_plan_doc {
//             //     SubscriptionPlan {
//             //         plan_type: sub_plan
//             //             .get_str("plan_type")
//             //             .unwrap_or_default()
//             //             .to_string(),
//             //         start_date: sub_plan
//             //             .get_datetime("start_date")
//             //             .ok()
//             //             .map(|dt| dt.to_owned()), // Convert to ISO 8601 formatted string
//             //         end_date: sub_plan
//             //             .get_datetime("end_date")
//             //             .ok()
//             //             .map(|dt| dt.to_owned()), // Convert to ISO 8601 formatted string
//             //         payment_status: sub_plan
//             //             .get_str("payment_status")
//             //             .ok()
//             //             .map(|s| s.to_string().to_owned()),
//             //     }
//             // } else {
//             //     SubscriptionPlan {
//             //         plan_type: "".to_string(),
//             //         start_date: None,
//             //         end_date: None,
//             //         payment_status: None,
//             //     }
//             // };

//             // // Handle usage (nested object)
//             // let usage_doc = user_doc.get_document("usage").ok();
//             // let usage = if let Some(usage_data) = usage_doc {
//             //     Usage {
//             //         tries_used: usage_data.get_i32("tries_used").unwrap_or_default(),
//             //         max_tries: usage_data.get_i32("max_tries").unwrap_or_default(),
//             //     }
//             // } else {
//             //     Usage {
//             //         tries_used: 0,
//             //         max_tries: 0,
//             //     }
//             // };

//             // let user_response = UserResponse {
//             //     id: user_doc
//             //         .get_object_id("_id")
//             //         .map(|oid| oid.to_hex())
//             //         .unwrap_or_default(),
//             //     name: user_doc.get_str("name").unwrap_or_default().to_string(),
//             //     email: user_doc.get_str("email").unwrap_or_default().to_string(),
//             //     google_id: user_doc.get_str("google_id").map(|s| s.to_string()).ok(),
//             //     login_type: user_doc.get_str("login_type").map(|s| s.to_string()).ok(),
//             //     profile_img: user_doc.get_str("profileImg").map(|s| s.to_string()).ok(),
//             //     subscription_plan, // Use the subscription_plan we built above
//             //     usage,             // Use the usage object we built above
//             // };

//             // Return the user data
//             (
//                 StatusCode::OK,
//                 Json(json!({ "success": true, "data": user_doc })),
//             )
//         }
//         Ok(None) => {
//             // User not found
//             (
//                 StatusCode::NOT_FOUND,
//                 Json(json!({ "success": false, "message": "User not found" })),
//             )
//         }
//         Err(e) => {
//             // Error during database lookup
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({ "success": false, "message": e.to_string() })),
//             )
//         }
//     }
// }
