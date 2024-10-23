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
use chrono::{DateTime, TimeZone, Utc};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    change_stream::session,
};
// use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
// use mongodb::Collection;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use stripe::{
    CheckoutSession, CheckoutSessionPaymentStatus, CheckoutSessionStatus, Client,
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionPaymentMethodTypes,
    CustomerId, IdOrCreate, ListCheckoutSessions, ListCheckoutSessionsCustomerDetails, ListPrices,
    ListSubscriptionItems, ListSubscriptions, Price, Subscription, SubscriptionId,
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
            StatusCode::BAD_REQUEST,
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
        Ok(id) => {
            let mut update_fields = Document::new();
            update_fields.insert("subscription_id", id.to_hex());

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
) -> Result<ObjectId, String> {
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

        // Extracting subscription data .
        let subscription_data = Subscription::retrieve(
            &client,
            &"sub_1QAqRtRtqMxXmkr4l4tkiEZC".parse().unwrap(),
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
            user_id: user_id,
            stripe_customer_id: customer_id,
            plan_details: PlanDetails {
                plan_id: plan_id,
                product_id: product_id,
                plan_name: plan_name,
                billing_cycle: billing_cycle,
                start_date: timestamp_to_utc(subscription_data.current_period_start),
                end_date: timestamp_to_utc(subscription_data.current_period_end),
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
                payment_date: Utc::now(),
            }],
        };

        // log::info!("new subscription data : {:#?}", new_subscription);

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
        return Ok(subscription_id);
    }
    return Err("Plan verification unsuccessful".to_string());
}

// Helper function to convert timestamp to DateTime<Utc>
fn timestamp_to_utc(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(timestamp, 0).unwrap()
}
// -------------------------------------------------------------------------------------------------------------------
// INFO - checkout session data : CheckoutSession {
//     id: CheckoutSessionId(
//         "cs_test_a1Owo7ibMsTCOjMWA0kMwUr8jAVqE539zKHPOCt7KtBK5oCb5XgQzW4xh7",
//     ),
//     after_expiration: None,
//     allow_promotion_codes: None,
//     amount_subtotal: Some(
//         29999,
//     ),
//     amount_total: Some(
//         29999,
//     ),
//     automatic_tax: PaymentPagesCheckoutSessionAutomaticTax {
//         enabled: false,
//         liability: None,
//         status: None,
//     },
//     billing_address_collection: None,
//     cancel_url: Some(
//         "http://localhost:5170/verify?success=false&session_id={CHECKOUT_SESSION_ID}",
//     ),
//     client_reference_id: None,
//     client_secret: None,
//     consent: None,
//     consent_collection: None,
//     created: 1729158984,
//     currency: Some(
//         USD,
//     ),
//     currency_conversion: None,
//     custom_fields: [],
//     custom_text: PaymentPagesCheckoutSessionCustomText {
//         after_submit: None,
//         shipping_address: None,
//         submit: None,
//         terms_of_service_acceptance: None,
//     },
//     customer: Some(
//         Id(
//             CustomerId(
//                 "cus_R2wKbdOIQC2vnI",
//             ),
//         ),
//     ),
//     customer_creation: Some(
//         Always,
//     ),
//     customer_details: Some(
//         PaymentPagesCheckoutSessionCustomerDetails {
//             address: Some(
//                 Address {
//                     city: None,
//                     country: Some(
//                         "IN",
//                     ),
//                     line1: None,
//                     line2: None,
//                     postal_code: None,
//                     state: None,
//                 },
//             ),
//             email: Some(
//                 "user0@gmail.com",
//             ),
//             name: Some(
//                 "User0",
//             ),
//             phone: None,
//             tax_exempt: Some(
//                 None,
//             ),
//             tax_ids: Some(
//                 [],
//             ),
//         },
//     ),
//     customer_email: Some(
//         "user0@gmail.com",
//     ),
//     expires_at: 1729245384,
//     invoice: Some(
//         Id(
//             InvoiceId(
//                 "in_1QAqRtRtqMxXmkr47dsB3yCn",
//             ),
//         ),
//     ),
//     invoice_creation: None,
//     line_items: None,
//     livemode: false,
//     locale: None,
//     metadata: Some(
//         {},
//     ),
//     mode: Subscription,
//     payment_intent: None,
//     payment_link: None,
//     payment_method_collection: Some(
//         Always,
//     ),
//     payment_method_configuration_details: None,
//     payment_method_options: Some(
//         CheckoutSessionPaymentMethodOptions {
//             acss_debit: None,
//             affirm: None,
//             afterpay_clearpay: None,
//             alipay: None,
//             au_becs_debit: None,
//             bacs_debit: None,
//             bancontact: None,
//             boleto: None,
//             card: Some(
//                 CheckoutCardPaymentMethodOptions {
//                     installments: None,
//                     setup_future_usage: None,
//                     statement_descriptor_suffix_kana: None,
//                     statement_descriptor_suffix_kanji: None,
//                 },
//             ),
//             cashapp: None,
//             customer_balance: None,
//             eps: None,
//             fpx: None,
//             giropay: None,
//             grabpay: None,
//             ideal: None,
//             klarna: None,
//             konbini: None,
//             link: None,
//             oxxo: None,
//             p24: None,
//             paynow: None,
//             paypal: None,
//             pix: None,
//             revolut_pay: None,
//             sepa_debit: None,
//             sofort: None,
//             swish: None,
//             us_bank_account: None,
//         },
//     ),
//     payment_method_types: [
//         "card",
//         "paypal",
//     ],
//     payment_status: Paid,
//     phone_number_collection: Some(
//         PaymentPagesCheckoutSessionPhoneNumberCollection {
//             enabled: false,
//         },
//     ),
//     recovered_from: None,
//     redirect_on_completion: None,
//     return_url: None,
//     setup_intent: None,
//     shipping_address_collection: None,
//     shipping_cost: None,
//     shipping_details: None,
//     shipping_options: [],
//     status: Some(
//         Complete,
//     ),
//     submit_type: None,
//     subscription: Some(
//         Id(
//             SubscriptionId(
//                 "sub_1QAqRtRtqMxXmkr4l4tkiEZC",
//             ),
//         ),
//     ),
//     success_url: Some(
//         "http://localhost:5170/verify?success=true&session_id={CHECKOUT_SESSION_ID}",
//     ),
//     tax_id_collection: None,
//     total_details: Some(
//         PaymentPagesCheckoutSessionTotalDetails {
//             amount_discount: 0,
//             amount_shipping: Some(
//                 0,
//             ),
//             amount_tax: 0,
//             breakdown: None,
//         },
//     ),
//     ui_mode: Some(
//         Hosted,
//     ),
//     url: None,
// }

//----------------------------------------------------------------------------------------------------------------------------------

// INFO - subscription data : Subscription {
//     id: SubscriptionId(
//         "sub_1QAqRtRtqMxXmkr4l4tkiEZC",
//     ),
//     application: None,
//     application_fee_percent: None,
//     automatic_tax: SubscriptionAutomaticTax {
//         enabled: false,
//         liability: None,
//     },
//     billing_cycle_anchor: 1729159013,
//     billing_cycle_anchor_config: None,
//     billing_thresholds: None,
//     cancel_at: None,
//     cancel_at_period_end: false,
//     canceled_at: None,
//     cancellation_details: Some(
//         CancellationDetails {
//             comment: None,
//             feedback: None,
//             reason: None,
//         },
//     ),
//     collection_method: Some(
//         ChargeAutomatically,
//     ),
//     created: 1729159013,
//     currency: USD,
//     current_period_end: 1760695013,
//     current_period_start: 1729159013,
//     customer: Id(
//         CustomerId(
//             "cus_R2wKbdOIQC2vnI",
//         ),
//     ),
//     days_until_due: None,
//     default_payment_method: Some(
//         Id(
//             PaymentMethodId(
//                 "pm_1QAqRsRtqMxXmkr4RBUNJe2B",
//             ),
//         ),
//     ),
//     default_source: None,
//     default_tax_rates: Some(
//         [],
//     ),
//     description: None,
//     discount: None,
//     ended_at: None,
//     items: List {
//         data: [
//             SubscriptionItem {
//                 id: SubscriptionItemId(
//                     "si_R2wKDNoSXcv8zA",
//                 ),
//                 billing_thresholds: None,
//                 created: Some(
//                     1729159014,
//                 ),
//                 deleted: false,
//                 metadata: Some(
//                     {},
//                 ),
//                 plan: Some(
//                     Plan {
//                         id: PlanId(
//                             "price_1Q9l0eRtqMxXmkr4f7i32obw",
//                         ),
//                         active: Some(
//                             true,
//                         ),
//                         aggregate_usage: None,
//                         amount: Some(
//                             29999,
//                         ),
//                         amount_decimal: Some(
//                             "29999",
//                         ),
//                         billing_scheme: Some(
//                             PerUnit,
//                         ),
//                         created: Some(
//                             1728899776,
//                         ),
//                         currency: Some(
//                             USD,
//                         ),
//                         deleted: false,
//                         interval: Some(
//                             Year,
//                         ),
//                         interval_count: Some(
//                             1,
//                         ),
//                         livemode: Some(
//                             false,
//                         ),
//                         metadata: Some(
//                             {},
//                         ),
//                         nickname: None,
//                         product: Some(
//                             Id(
//                                 ProductId(
//                                     "prod_R1odaLLB9vHBxh",
//                                 ),
//                             ),
//                         ),
//                         tiers: None,
//                         tiers_mode: None,
//                         transform_usage: None,
//                         trial_period_days: None,
//                         usage_type: Some(
//                             Licensed,
//                         ),
//                     },
//                 ),
//                 price: Some(
//                     Price {
//                         id: PriceId(
//                             "price_1Q9l0eRtqMxXmkr4f7i32obw",
//                         ),
//                         active: Some(
//                             true,
//                         ),
//                         billing_scheme: Some(
//                             PerUnit,
//                         ),
//                         created: Some(
//                             1728899776,
//                         ),
//                         currency: Some(
//                             USD,
//                         ),
//                         currency_options: None,
//                         custom_unit_amount: None,
//                         deleted: false,
//                         livemode: Some(
//                             false,
//                         ),
//                         lookup_key: None,
//                         metadata: Some(
//                             {},
//                         ),
//                         nickname: None,
//                         product: Some(
//                             Object(
//                                 Product {
//                                     id: ProductId(
//                                         "prod_R1odaLLB9vHBxh",
//                                     ),
//                                     active: Some(
//                                         true,
//                                     ),
//                                     created: Some(
//                                         1728899775,
//                                     ),
//                                     default_price: Some(
//                                         Id(
//                                             PriceId(
//                                                 "price_1Q9l0eRtqMxXmkr4f7i32obw",
//                                             ),
//                                         ),
//                                     ),
//                                     deleted: false,
//                                     description: Some(
//                                         "Thankyou For buying this plan ,Enjoy Our Service",
//                                     ),
//                                     features: Some(
//                                         [],
//                                     ),
//                                     images: Some(
//                                         [],
//                                     ),
//                                     livemode: Some(
//                                         false,
//                                     ),
//                                     metadata: Some(
//                                         {},
//                                     ),
//                                     name: Some(
//                                         "Saas Pro Yearly",
//                                     ),
//                                     package_dimensions: None,
//                                     shippable: None,
//                                     statement_descriptor: None,
//                                     tax_code: None,
//                                     type_: Some(
//                                         Service,
//                                     ),
//                                     unit_label: None,
//                                     updated: Some(
//                                         1728976390,
//                                     ),
//                                     url: None,
//                                 },
//                             ),
//                         ),
//                         recurring: Some(
//                             Recurring {
//                                 aggregate_usage: None,
//                                 interval: Year,
//                                 interval_count: 1,
//                                 trial_period_days: None,
//                                 usage_type: Licensed,
//                             },
//                         ),
//                         tax_behavior: Some(
//                             Unspecified,
//                         ),
//                         tiers: None,
//                         tiers_mode: None,
//                         transform_quantity: None,
//                         type_: Some(
//                             Recurring,
//                         ),
//                         unit_amount: Some(
//                             29999,
//                         ),
//                         unit_amount_decimal: Some(
//                             "29999",
//                         ),
//                     },
//                 ),
//                 quantity: Some(
//                     1,
//                 ),
//                 subscription: Some(
//                     "sub_1QAqRtRtqMxXmkr4l4tkiEZC",
//                 ),
//                 tax_rates: Some(
//                     [],
//                 ),
//             },
//         ],
//         has_more: false,
//         total_count: Some(
//             1,
//         ),
//         url: "/v1/subscription_items?subscription=sub_1QAqRtRtqMxXmkr4l4tkiEZC",
//     },
//     latest_invoice: Some(
//         Id(
//             InvoiceId(
//                 "in_1QAqRtRtqMxXmkr47dsB3yCn",
//             ),
//         ),
//     ),
//     livemode: false,
//     metadata: {},
//     next_pending_invoice_item_invoice: None,
//     on_behalf_of: None,
//     pause_collection: None,
//     payment_settings: Some(
//         SubscriptionsResourcePaymentSettings {
//             payment_method_options: Some(
//                 SubscriptionsResourcePaymentMethodOptions {
//                     acss_debit: None,
//                     bancontact: None,
//                     card: Some(
//                         SubscriptionPaymentMethodOptionsCard {
//                             mandate_options: None,
//                             network: None,
//                             request_three_d_secure: Some(
//                                 Automatic,
//                             ),
//                         },
//                     ),
//                     customer_balance: None,
//                     konbini: None,
//                     us_bank_account: None,
//                 },
//             ),
//             payment_method_types: None,
//             save_default_payment_method: Some(
//                 Off,
//             ),
//         },
//     ),
//     pending_invoice_item_interval: None,
//     pending_setup_intent: None,
//     pending_update: None,
//     schedule: None,
//     start_date: 1729159013,
//     status: Active,
//     test_clock: None,
//     transfer_data: None,
//     trial_end: None,
//     trial_settings: Some(
//         SubscriptionsTrialsResourceTrialSettings {
//             end_behavior: SubscriptionsTrialsResourceEndBehavior {
//                 missing_payment_method: CreateInvoice,
//             },
//         },
//     ),
//     trial_start: None,
// }
