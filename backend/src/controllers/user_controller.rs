// use reqwest::Client;
// use sha2::Digest;
// use std::collections::HashMap;
// use std::time::{SystemTime, UNIX_EPOCH};
// use sha2::Sha256;

// use axum::http::{HeaderMap, HeaderValue};
// // use cookie::{time::Duration as OtherDuration, Cookie};
// use cookie::{Cookie, SameSite};

// use axum_extra::extract::cookie::CookieJar;
// use cookie::time::Duration as OtherDuration; // Use from axum_extra

use crate::configration::gett;
use crate::middleware::auth::Claimss;
use crate::models::subscription_model::SubscriptionPlan;
use crate::models::user_model::{ActivityLog, User};
use crate::utils::generate_token::{
    generate_token_and_set_cookie, generate_token_and_unset_cookie,
};
use axum::response::Response;
use axum::{
    // extract::Form,
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use bson::Document;
use chrono::{FixedOffset, Utc};
use regex::Regex;
// use chrono::{Duration, Utc};

use cloudinary::upload::result::UploadResult;
// use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::{doc, oid::ObjectId, Bson, DateTime as BsonDateTime};

use cloudinary::upload::{Source, Upload, UploadOptions};
use serde::{Deserialize, Serialize};
use serde_json::json;

use validator::validate_email;

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUserData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "currentPassword")]
    pub current_password: Option<String>,
    #[serde(rename = "newPassword")]
    pub new_password: Option<String>,
    #[serde(rename = "profileImg")]
    pub profile_img: Option<String>,
    pub tries_used: Option<i32>,
    pub activity_log_num: Option<i32>,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct UserResponse {
//     #[serde(rename = "_id")]
//     pub id: String, // User's ID
//     pub name: String,                        // User's name
//     pub email: String,                       // User's email
//     pub google_id: Option<String>,           // Optional google_id
//     pub login_type: Option<String>,          // Optional login type
//     pub profile_img: Option<String>,         // Optional profile image
//     pub subscription_plan: SubscriptionPlan, // User's subscription plan details
//     pub usage: Usage,                        // User's usage data
// }

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
        "subscription_id": None::<Bson>,
        "user_address":{
             "address": {
                 "city": "",        // City
                 "country": "",     // Country
                 "line1": "",       // Address line 1
                 "line2": "",       // Address line 2
                 "postal_code": "", // Postal code
                 "state": "",       // State or province
             },
             "email": "",        // User email
             "name": "",         // User name
             "phone": "",        // User phone
        },
        "usage": {
            "tries_used": 0, // Start with 0 tries used
            "max_tries": 10,  // Basic plan allows 10 tries
        },
        "activity_log": None::<Bson>,
        "billing_history":None::<Bson>,
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

    let headers = generate_token_and_set_cookie(user_id).await;

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
    // log::info!("accessing login user function.");
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

    let headers = generate_token_and_set_cookie(user_id).await;

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

pub async fn logout_user(Extension(claims): Extension<Claimss>) -> Response {
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

            // Retrieve the subscription_id and convert to ObjectId
            if let Some(subscription_id) = user_doc
                .get_str("subscription_id")
                .ok()
                .and_then(|s| ObjectId::parse_str(s).ok())
            {
                let subscription_collection = SubscriptionPlan::get_subscription_collection().await;
                if let Ok(Some(mut subscription_doc)) = subscription_collection
                    .find_one(doc! { "_id": subscription_id })
                    .await
                {
                    if let Some(id) = subscription_doc.remove("_id") {
                        if let Some(object_id) = id.as_object_id() {
                            subscription_doc.insert("_id", object_id.to_string());
                        }
                    }
                    user_doc.insert("subscription_data", subscription_doc);
                } else {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({ "success": false, "message": "Subscription not found" })),
                    );
                }
            }

            // Return the user data with subscription details
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
    Json(form): Json<UpdateUserData>,
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

    // Assuming user_doc is a BSON document, extract the `profileImg` field
    let profile_img_old = user_doc.get("profileImg").and_then(|v| match v {
        Bson::String(url) => Some(url.clone()),
        _ => None,
    });

    // Create a document for the updates
    let mut update_fields = Document::new();
    // Update name if provided
    if let Some(name) = &form.name {
        if !name.is_empty() {
            update_fields.insert("name", name.clone());
        }
    }

    // Update email if provided
    if let Some(email) = &form.email {
        if !email.is_empty() {
            update_fields.insert("email", email.clone());
        }
    }
    // Update email if provided
    if let Some(username) = &form.username {
        if !username.is_empty() {
            update_fields.insert("username", username.clone());
        }
    }

    // Update tries_used if provided
    if let Some(tries_used) = form.tries_used {
        update_fields.insert("usage.tries_used", tries_used);
    }

    // Update profile image if provided
    if let Some(profile_img) = &form.profile_img {
        let update_profile_img_result = match upload_image_to_cloudinary(profile_img).await {
            Ok(url) => Ok(url),
            Err(e) => Err(e.to_string()),
        };

        match update_profile_img_result {
            Ok(url) => {
                // println!(" secure url : {:?}", url);
                if let Some(profile_img_old) = profile_img_old {
                    let _destroy_old_img = match destroy_profile_image(&profile_img_old).await {
                        Ok(msg) => Ok(msg),
                        Err(e) => Err(e.to_string()),
                    };
                }
                update_fields.insert("profileImg", url);
            }
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "success": false, "message": e })),
                )
            }
        }
    }

    // Handle password change
    if let (Some(current), Some(new)) = (&form.current_password, &form.new_password) {
        if current.is_empty() && new.is_empty() {
            // Don't update the password if all three fields are empty
        } else if current.is_empty() || new.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Any password fields cannot be empty" })),
            );
        } else {
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
    }

    // Handle activity log update
    if let Some(act_num) = form.activity_log_num {
        let new_act_log = User::create_activity_log(act_num).await;

        // Fetch the existing activity_log and prepend the new log entry
        let mut updated_activity_log = match user_doc.get_array("activity_log") {
            Ok(activity_log) => {
                // Convert BSON array to Vec<ActivityLog> if it exists
                activity_log
                    .iter()
                    .filter_map(|doc| bson::from_bson::<ActivityLog>(doc.clone()).ok())
                    .collect::<Vec<ActivityLog>>()
            }
            Err(_) => vec![], // If no activity log exists, initialize with an empty vector
        };

        // Insert the new activity log at the beginning
        updated_activity_log.insert(0, new_act_log);

        // Convert updated_activity_log back to BSON and insert into update fields
        let bson_activity_log = updated_activity_log
            .iter()
            .map(|log| bson::to_bson(log))
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default();
        update_fields.insert("activity_log", bson_activity_log);
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

pub async fn upload_image_to_cloudinary(profile_img: &str) -> Result<String, String> {
    let cloud_name = gett::<String>("CLOUDINARY_CLOUD_NAME");
    let api_key = gett::<String>("CLOUDINARY_API_KEY");
    let api_secret = gett::<String>("CLOUDINARY_API_SECRET");

    let options = UploadOptions::new();
    let upload = Upload::new(
        api_key.to_string(),
        cloud_name.to_string(),
        api_secret.to_string(),
    );

    // Upload the image to Cloudinary
    let upload_result = upload
        .image(Source::DataUrl(profile_img.into()), &options)
        .await;

    // log::info!("Upload result :: , {:?}", upload_result);

    match upload_result {
        // If successful, return the secure URL
        Ok(result) => {
            // Match the result to see if it contains the expected variant
            match result {
                UploadResult::Response(_) => {
                    todo!()
                }
                UploadResult::ResponseWithImageMetadata(response) => {
                    // If the result contains the image metadata, return the secure URL
                    Ok(response.secure_url)
                }
                UploadResult::Error(error) => {
                    // Handle the case where there is an error in the upload
                    Err(format!("Upload error: {:?}", error))
                }
            }
        }
        // Handle errors
        Err(e) => {
            // Attempt to parse and extract the secure_url from the error message
            let error_message = format!("{:?}", e);
            log::error!("{}", error_message);

            // Use regex to find the secure_url in the error message
            let re = Regex::new(r#""secure_url":"(https://[^"]+)""#).unwrap();
            if let Some(captures) = re.captures(&error_message) {
                if let Some(secure_url) = captures.get(1) {
                    return Ok(secure_url.as_str().to_string());
                }
            }

            Err("Failed to extract secure_url from error".to_string())
        }
    }
}

pub async fn destroy_profile_image(profile_img_url: &str) -> Result<String, String> {
    // Cloudinary configuration
    let cloud_name = gett::<String>("CLOUDINARY_CLOUD_NAME");
    let api_key = gett::<String>("CLOUDINARY_API_KEY");
    let api_secret = gett::<String>("CLOUDINARY_API_SECRET");

    // Extract public_id from the profile_img_url
    let re = Regex::new(r"/v\d+/(.+)\.[a-zA-Z]+$").unwrap(); // Extracts the part between `/v<digits>/` and the file extension
    let pub_id = match re.captures(profile_img_url) {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()).to_string(),
        None => {
            return Err("Failed to extract public_id from the URL".to_string());
        }
    };

    // Cloudinary upload configuration for destroy action
    let upload = Upload::new(
        api_key.to_string(),
        cloud_name.to_string(),
        api_secret.to_string(),
    );

    // Perform the destroy action for the public_id
    let upload_result = match upload.destroy(pub_id).await {
        Ok(result) => {
            // Check if the image was destroyed successfully
            if result.result == "ok" {
                // log::info!("Image destroyed successfully");
                Ok("Image destroyed successfully".to_string())
            } else {
                Err(format!("Failed to destroy image: {:?}", result))
            }
        }
        Err(e) => Err(format!("Failed to destroy image: {}", e)),
    };

    upload_result
}

// destory the image .

// pub async fn destroy_profile_image_curl(profile_img_url: &str) -> Result<String, String> {
//     // Cloudinary configuration
//     let cloud_name = gett::<String>("CLOUDINARY_CLOUD_NAME");
//     let api_key = gett::<String>("CLOUDINARY_API_KEY");
//     let api_secret = gett::<String>("CLOUDINARY_API_SECRET");

//     // Extract public_id from the profile_img_url
//     let re = Regex::new(r"/v\d+/(.+)\.[a-zA-Z]+$").unwrap(); // Extracts the part between `/v<digits>/` and the file extension
//     let pub_id = match re.captures(profile_img_url) {
//         Some(caps) => caps.get(1).map_or("", |m| m.as_str()).to_string(),
//         None => {
//             return Err("Failed to extract public_id from the URL".to_string());
//         }
//     };

//     // Generate timestamp (current Unix time in seconds)
//     let start = SystemTime::now();
//     let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
//     let timestamp_str = timestamp.to_string();

//     // Create the string to sign using the public_id, timestamp, and secret
//     let payload = format!("public_id={}&timestamp={}{}", pub_id, timestamp, api_secret);

//     // Generate the signature using SHA-256
//     let mut hasher = Sha256::new();
//     hasher.update(payload.as_bytes());
//     let signature = hex::encode(hasher.finalize());

//     // Prepare the form data for the Cloudinary destroy API
//     let form = HashMap::from([
//         ("public_id", pub_id.as_str()),   // Extracted public_id
//         ("api_key", &api_key),            // Cloudinary API key
//         ("timestamp", &timestamp_str),    // Timestamp
//         ("signature", &signature),        // Generated signature
//     ]);

//     // Perform the POST request to Cloudinary destroy endpoint
//     let client = Client::new();
//     let url = format!(
//         "https://api.cloudinary.com/v1_1/{}/image/destroy",
//         cloud_name
//     );
//     let response = match client.post(&url).form(&form).send().await {
//         Ok(res) => res,
//         Err(err) => {
//             return Err(format!("Failed to send request to Cloudinary: {}", err));
//         }
//     };

//     // Check if the response is successful
//     if response.status().is_success() {
//         let res_text = response.text().await.unwrap_or_default();
//         println!("Response from Cloudinary: {}", res_text);
//         Ok("Image destroyed successfully".to_string())
//     } else {
//         let res_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
//         println!("Failed to destroy image: {}", res_text);
//         Err(format!("Failed to destroy image: {}", res_text))
//     }
// }

// pub async fn upload_profile_image(profile_img: &str) -> Result<String, Box<dyn std::error::Error>> {
//     // Cloudinary configuration
//     let cloud_name = gett::<String>("CLOUDINARY_CLOUD_NAME");
//     let api_key = gett::<String>("CLOUDINARY_API_KEY");
//     let api_secret = gett::<String>("CLOUDINARY_API_SECRET");

//     // Generate timestamp (current Unix time in seconds)
//     let start = SystemTime::now();
//     let timestamp = start.duration_since(UNIX_EPOCH)?.as_secs();
//     let timestamp_str = timestamp.to_string();

//     // Create the string to sign using only the timestamp and the secret
//     let payload = format!("timestamp={}{}", timestamp, api_secret); // Include api_secret directly in the string to be hashed

//     // Generate the signature using SHA-256
//     let mut hasher = Sha256::new();
//     hasher.update(payload.as_bytes());
//     let signature = hex::encode(hasher.finalize());

//     // Prepare the multipart form data
//     let form = HashMap::from([
//         ("file", profile_img), // Data URL (Base64 encoded)
//         ("media_metadata", "true"),
//         ("api_key", &api_key),         // API key
//         ("timestamp", &timestamp_str), // Timestamp
//         ("signature", &signature),     // Correct signature
//     ]);

//     // Perform the POST request to Cloudinary
//     let client = Client::new();
//     let url = format!(
//         "https://api.cloudinary.com/v1_1/{}/image/upload",
//         cloud_name
//     );
//     let response = client.post(&url).form(&form).send().await?;

//     // Print the response for debugging purposes
//     println!("Response .. {:?}", response);

//     // Get the URL of the uploaded image from the response (assuming it's in JSON)
//     // let uploaded_url = response.secure_url;
//     // println!("Uploaded image URL: {}", uploaded_url);

//     Ok("abc".to_string())
// }
