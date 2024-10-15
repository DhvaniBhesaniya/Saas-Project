use crate::configration::gett;
use crate::middleware::auth::Claimss;
use crate::models::user_model::User;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};

use bson::Bson;
use chrono::{DateTime, Utc};
use mongodb::bson::{self, doc, oid::ObjectId, Document};
// use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
// use mongodb::Collection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stripe::{
    CheckoutSession, CheckoutSessionPaymentStatus, CheckoutSessionStatus, Client,
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionPaymentMethodTypes,
    ListCheckoutSessions, ListCheckoutSessionsCustomerDetails, ListPrices, Price,
};

#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    price_id: String,
}
#[derive(Debug, Deserialize)]
pub struct VerifyOrderRequest {
    session_id: String,
}

pub async fn buy_plan(
    Extension(claims): Extension<Claimss>, // Assuming 'claims' contains the user ID
    Json(req): Json<PlaceOrderRequest>,
) -> impl IntoResponse {
    let local_frontend_url = gett::<String>("local_frontend_url");
    let frontend_url = local_frontend_url;

    let stripe_key = gett::<String>("stripe_secret_key");
    let client = Client::new(stripe_key);

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
    let user_email = user_doc.get("email").and_then(|v| match v {
        Bson::String(e) => Some(e.clone()),
        _ => None,
    });

    // Example of using ListProducts with specific parameters
    let list_price_params = ListPrices {
        active: Some(true), // Only return active products
        created: None,
        limit: Some(10),      // Limit the number of products to 10
        expand: &[], // You can specify fields to expand if needed, leave it empty otherwise
        ..Default::default()  // Fill in other fields as default
    };

    let price_list = Price::list(&client, &list_price_params).await.unwrap();

    // Find the requested price ID in the fetched price list
    let matched_price = price_list
        .data
        .iter()
        .find(|p| p.id.to_string() == req.price_id);

    // If the price doesn't exist, return an error
    if matched_price.is_none() {
        return (
            StatusCode::OK,
            Json(json!({ "success": false, "message": "Plan doesn't exist" })),
        );
    }

    // Prepare the line item for the checkout session
    let line_item = CreateCheckoutSessionLineItems {
        price: Some(req.price_id.to_string()),
        quantity: Some(1), // Assuming the quantity is 1 for buying a plan
        ..Default::default()
    };

    // // Create a checkout session
    let checkout_session_url = create_stripe_session(
        &client,
        line_item,
        frontend_url.clone(),
        user_email.to_owned(),
    )
    .await;

    // Return the session URL
    (
        StatusCode::OK,
        Json(json!({ "success": true, "session_url": checkout_session_url })),
    )
}

async fn create_stripe_session(
    client: &Client,
    line_items: CreateCheckoutSessionLineItems,
    frontend_url: String,
    user_email: Option<String>,
) -> String {
    // Generate the success and cancel URLs with the checkout session ID placeholder
    let cancel_url = format!(
        "{}/verify?success=false&session_id={{CHECKOUT_SESSION_ID}}",
        frontend_url
    );
    let success_url = format!(
        "{}/verify?success=true&session_id={{CHECKOUT_SESSION_ID}}",
        frontend_url
    );

    let useremail = match user_email {
        Some(email) => email,
        None => "abc@gmail.com".to_string(),
    };

    // Create the checkout session
    let checkout_session = {
        let mut params = CreateCheckoutSession::new();
        params.cancel_url = Some(&cancel_url);
        params.success_url = Some(&success_url);
        params.mode = Some(stripe::CheckoutSessionMode::Subscription); // Assuming payment mode
        params.payment_method_types = Some(vec![
            CreateCheckoutSessionPaymentMethodTypes::Card,
            CreateCheckoutSessionPaymentMethodTypes::Paypal,
        ]);
        params.line_items = Some(vec![line_items]); // Pass the single line item
        params.expand = &["line_items", "line_items.data.price.product"]; // Optionally expand line item fields
        params.customer_email = Some(&useremail);

        stripe::CheckoutSession::create(client, params)
            .await
            .unwrap()
    };

    checkout_session.url.unwrap()
}

pub async fn verify_plan(
    Extension(claims): Extension<Claimss>, // Assuming 'claims' contains the user ID
    Json(req): Json<VerifyOrderRequest>,
) -> impl IntoResponse {
    let stripe_key = gett::<String>("stripe_secret_key");
    let client = Client::new(stripe_key);

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
                StatusCode::OK,
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
    let user_email = user_doc.get("email").and_then(|v| match v {
        Bson::String(e) => Some(e.clone()),
        _ => None,
    });

    // Example of using ListProducts with specific parameters
    let list_price_params = ListCheckoutSessions {
        customer_details: Some(ListCheckoutSessionsCustomerDetails {
            email: user_email.unwrap(),
        }),
        status: Some(CheckoutSessionStatus::Complete),
        ..Default::default() // Fill in other fields as default
    };

    let session_list = CheckoutSession::list(&client, &list_price_params)
        .await
        .unwrap();

    // log::info!("checkout session data : {:#?}", session_list);

    let session_id = req.session_id.clone();

    // Find the session with the matching session ID
    let matched_session = session_list
        .data
        .iter()
        .find(|s| s.id.to_string() == session_id);

    // If no matching session is found, return an error
    if matched_session.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "No session found" })),
        );
    }

    // log::info!("checkout session data : {:#?}", matched_session.unwrap());
    // Check if the status is 'complete' and payment_status is 'paid'
    if matched_session.unwrap().status == Some(CheckoutSessionStatus::Complete)
        && matched_session.unwrap().payment_status == CheckoutSessionPaymentStatus::Paid
    {
        // Plan verified successfully
        return (
            StatusCode::OK,
            Json(json!({ "success": true, "message": "Plan verified successfully" })),
        );
    }

    // If the status and payment_status do not match the expected values
    (
        StatusCode::OK,
        Json(json!({ "success": false, "message": "Plan verification unsuccessful" })),
    )
}
