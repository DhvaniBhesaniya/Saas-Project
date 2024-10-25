use crate::middleware::auth::Claimss;
use crate::models::subscription_model::{PaymentDetails, PlanDetails};
use crate::models::user_model::User;
use crate::{configration::gett, models::subscription_model::SubscriptionPlan};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};

use bson::Bson;
// use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use mongodb::bson::{self, doc, oid::ObjectId, DateTime as BsonDateTime, Document};
// use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
// use mongodb::Collection;
use serde::Deserialize;
use serde_json::{json, Value};
use stripe::{
    CheckoutSession, CheckoutSessionPaymentStatus, CheckoutSessionStatus, Client,
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionPaymentMethodTypes,
    ListPrices, Price, Subscription,
};

#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    price_id: String,
}
#[derive(Debug, Deserialize)]
pub struct VerifyOrderRequest {
    session_id: String,
}

#[derive(Debug, PartialEq)]
pub enum PlanType {
    SaasEnterpriceYearly { value: i32 },
    SaasProYearly { value: i32 },
    SaasEnterpriceMonthly { value: i32 },
    SaasProMonthly { value: i32 },
}

impl PlanType {
    pub fn get_value(s: &str) -> Option<Self> {
        match s {
            "Saas Enterprice Yearly" => Some(PlanType::SaasEnterpriceYearly { value: 1000 }),
            "Saas Pro Yearly" => Some(PlanType::SaasProYearly { value: 500 }),
            "Saas Enterprice Monthly" => Some(PlanType::SaasEnterpriceMonthly { value: 500 }),
            "Saas Pro Monthly" => Some(PlanType::SaasProMonthly { value: 250 }),
            _ => None,
        }
    }
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
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "Plan doesn't exist" })),
        );
    }

    // Prepare the line item for the checkout session
    let line_item = CreateCheckoutSessionLineItems {
        price: Some(req.price_id.to_string()),
        quantity: Some(1), // The quantity is 1 for buying a plan
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

    // log::info!("stripe scheckout session. : {:#?}",checkout_session);

    checkout_session.url.unwrap()
}

pub async fn verify_plan(
    Extension(claims): Extension<Claimss>, // Assuming 'claims' contains the user ID
    Json(req): Json<VerifyOrderRequest>,
) -> impl IntoResponse {
    let stripe_key = gett::<String>("stripe_secret_key");
    let client = Client::new(stripe_key);

    let user_collection = User::get_user_collection().await;
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
    let _user_doc = match user_collection.find_one(filter.clone()).await {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
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
    // let _user_email = user_doc.get("email").and_then(|v| match v {
    //     Bson::String(e) => Some(e.clone()),
    //     _ => None,
    // });

    let session_id = req.session_id.clone();

    let checkout_session =
        get_checkout_session_data_and_update(&client, session_id, user_id, None).await;

    match checkout_session {
        Ok(data) => {
            // Initialize variables for extracted values
            let mut sub_id: Option<ObjectId> = None;
            let mut max_usage: Option<i32> = None;

            // Extract sub_id as ObjectId
            if let Some(sub_id_value) = data.get("sub_id").and_then(|v| v.get("$oid")) {
                if let Some(sub_id_str) = sub_id_value.as_str() {
                    sub_id = ObjectId::parse_str(sub_id_str).ok();
                }
            }

            // Extract max_usage as i32
            if let Some(max_usage_value) = data.get("max_usage") {
                max_usage = max_usage_value.as_i64().map(|v| v as i32);
            }
            let mut update_fields = Document::new();
            // Insert into update fields if successfully extracted
            if let Some(sub_id) = sub_id {
                update_fields.insert("subscription_id", sub_id.to_hex());
            }
            if let Some(max_usage) = max_usage {
                update_fields.insert("usage.tries_used", 0);
                update_fields.insert("usage.max_tries", max_usage);
            }
            // Update the user document in MongoDB
            if !update_fields.is_empty() {
                let update_doc = doc! { "$set": update_fields };
                if user_collection
                    .update_one(filter, update_doc)
                    .await
                    .is_err()
                {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            json!({ "success": false, "message": "Failed to update user data while veriying." }),
                        ),
                    );
                }
            }
            return (
                StatusCode::OK,
                Json(json!({ "success": true, "message": "Plan verified successfully" })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": e })),
            )
        }
    };
}

pub async fn get_checkout_session_data_and_update(
    client: &Client,
    session_id: String,
    user_id: ObjectId,
    _user_email: Option<String>,
) -> Result<Value, String> {
    // method 1:  gett session list and find it

    // // Example of using ListProducts with specific parameters
    // let list_price_params = ListCheckoutSessions {
    //     customer_details: Some(ListCheckoutSessionsCustomerDetails {
    //         email: user_email.unwrap(),
    //     }),
    //     status: Some(CheckoutSessionStatus::Complete),
    //     ..Default::default() // Fill in other fields as default
    // };

    // let session_list = CheckoutSession::list(&client, &list_price_params)
    //     .await
    //     .unwrap();

    // // Find the session with the matching session ID
    // let matched_session = session_list
    //     .data
    //     .iter()
    //     .find(|s| s.id.to_string() == session_id);

    // // If no matching session is found, return an error
    // if matched_session.is_none() {
    //     return Err("no matching session found".to_string());
    // }
    //       // Check if the status is 'complete' and payment_status is 'paid'
    //     if matched_session.unwrap().status == Some(CheckoutSessionStatus::Complete)
    //     && matched_session.unwrap().payment_status == CheckoutSessionPaymentStatus::Paid
    // {
    //     // Plan verified successfully
    //     return (
    //         StatusCode::OK,
    //         Json(json!({ "success": true, "message": "Plan verified successfully" })),
    //     );
    // }
    //-------------------------------------------------------------------------------
    // method 2: get the session id and retrive its data.
    let checkout_session_data =
        CheckoutSession::retrieve(&client, &session_id.parse().unwrap(), &[])
            .await
            .map_err(|err| err.to_string())?;

    // log::info!("checkout session data : {:#?}", checkout_session_data);

    // Check if the status is 'complete' and payment_status is 'paid'
    if checkout_session_data.status == Some(CheckoutSessionStatus::Complete)
        && checkout_session_data.payment_status == CheckoutSessionPaymentStatus::Paid
    {
        let customer_id = checkout_session_data
            .to_owned()
            .customer
            .unwrap()
            .id()
            .as_str()
            .to_string();

        let payment_method = match checkout_session_data
            .payment_method_options
            .unwrap()
            .card
            .is_some()
        {
            true => "Card".to_string(),
            false => "Unknown".to_string(),
        };

        let subscription_id = match checkout_session_data.subscription.unwrap(){
            stripe::Expandable::Id(sub_id) => sub_id.to_string(),
            _ => "Unknown".to_string(),
        };

        // Extracting subscription data .
        let subscription_data = Subscription::retrieve(
            &client,
            &subscription_id.parse().unwrap(),
            &["items", "items.data.price.product", "schedule"],
        )
        .await
        .unwrap();
        // log::info!("subscription data : {:#?}", subscription_data);

        let plan_name = subscription_data
            .items
            .data
            .get(0) // Assuming you want the first item
            .and_then(|item| item.price.as_ref()) // Get price object
            .and_then(|price| price.product.as_ref()) // Get product object
            .and_then(|product| match product {
                stripe::Expandable::Object(product_obj) => product_obj.name.clone(), // Extract plan name
                _ => None,
            })
            .unwrap_or("Unknown Plan".to_string()); // Default if not found

        let invoice_id = match subscription_data.latest_invoice {
            Some(stripe::Expandable::Id(invoice)) => invoice.to_string(),
            _ => "Unknown invoicd id".to_string(),
        };

        // Extract billing cycle
        let billing_cycle = subscription_data
            .items
            .data
            .get(0)
            .and_then(|item| item.price.as_ref())
            .and_then(|price| price.recurring.as_ref())
            .map(|recurring| match recurring.interval {
                stripe::RecurringInterval::Year => "yearly".to_string(),
                stripe::RecurringInterval::Month => "monthly".to_string(),
                stripe::RecurringInterval::Week => "weekly".to_string(),
                stripe::RecurringInterval::Day => "daily".to_string(),
            })
            .unwrap_or("Unknown Billing Cycle".to_string());

        // Extract plan ID and amount
        let price_data = subscription_data
            .items
            .data
            .get(0)
            .and_then(|item| item.price.as_ref());

        let plan_id = price_data
            .and_then(|price| Some(price.id.to_string())) // Extract plan ID as string
            .unwrap_or("Unknown Plan ID".to_string());

        // Product ID
        let product_id = price_data
            .and_then(|price| price.product.as_ref()) // Get product from price
            .and_then(|product| match product {
                stripe::Expandable::Object(product_obj) => Some(product_obj.id.to_string()), // Extract product ID
                stripe::Expandable::Id(id) => Some(id.to_string()), // If only ID is available
            })
            .unwrap_or("Unknown Product ID".to_string());

        let amount = price_data
            .and_then(|price| price.unit_amount) // Extract the amount
            .map(|amt| (amt as f64) / 100.0) // Convert to true amount (divide by 100)
            .unwrap_or(0.00); // Default if not found

        let new_subscription = SubscriptionPlan {
            id: None,
            stripe_subscription_id: subscription_data.id.as_str().to_string(),
            user_id: user_id.to_hex(),
            stripe_customer_id: customer_id,
            plan_details: PlanDetails {
                plan_id: plan_id,
                product_id: product_id,
                plan_name: plan_name.clone(),
                billing_cycle: billing_cycle,
                start_date: unix_to_bsontime(subscription_data.current_period_start),
                end_date: unix_to_bsontime(subscription_data.current_period_end),
            },
            auto_renew: true,
            refundable: false,
            status: subscription_data.status.to_string(),
            cancellation_date: None, // No cancellation yet
            payment_history: vec![PaymentDetails {
                invoice_id: invoice_id,
                payment_method: payment_method,
                currency: subscription_data.currency.to_string(),
                amount: amount,
                payment_date: unix_to_bsontime(checkout_session_data.created),
            }],
        };

        // log::info!("new subscription data : {:#?}", new_subscription);

        // let invoice_data = Invoice::retrieve(&client,&invoice_id.parse().unwrap(),&[]).await.unwrap();
        // log::info!("invoice data : {:#?}",invoice_data);

        // Get the MongoDB collection
        let collection = SubscriptionPlan::get_subscription_collection().await;

        // Convert the subscription plan to BSON
        let new_subscription_bson = mongodb::bson::to_document(&new_subscription).unwrap();

        // Insert the subscription plan into MongoDB
        let insert_result = collection.insert_one(new_subscription_bson).await.unwrap();

        // Extract the inserted `_id` from MongoDB insert result
        let subscription_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("Failed to get subscription _id");

        let plan_max_usage = PlanType::get_value(&plan_name).unwrap();
        let max_usage = match plan_max_usage {
            PlanType::SaasProYearly { value } => value,
            PlanType::SaasEnterpriceYearly { value } => value,
            PlanType::SaasProMonthly { value } => value,
            PlanType::SaasEnterpriceMonthly { value } => value
        };
        return Ok(json!({"sub_id":subscription_id,"max_usage":max_usage}));
    }
    return Err("Plan verification unsuccessful".to_string());
}

// Helper function to convert Unix to DateTime<Utc>
// fn timestamp_to_utc(timestamp: i64) -> DateTime<Utc> {
//     Utc.timestamp_opt(timestamp, 0).unwrap() //2024-10-17T09:56:53Z"  Utc format
// }
// Helper function to convert Unix (1729159014) to DateTime<Utc>
fn unix_to_bsontime(timestamp: i64) -> String {
    // println!("Input timestamp: {}", timestamp);

    let bson_dt = BsonDateTime::from_millis(timestamp * 1000);
    // println!("BsonDateTime raw: {:?}", bson_dt);
    return bson_dt.to_string();

    // let utc_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    // println!("UTC Time: {:?}", utc_time);

    // let formatted_string = utc_time
    //     .format("%Y-%m-%d %H:%M:%S.000 +00:00:00")
    //     .to_string();

    //     // Create an offset for UTC+5:30 (India)
    //     let india_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
    //     let local_time = utc_time.with_timezone(&india_offset);
    //     let formatted_string = local_time.format("%Y-%m-%d %H:%M:%S.000 %z").to_string();
    //     println!(" Indian time:  Formatted string: {}", formatted_string);
}
