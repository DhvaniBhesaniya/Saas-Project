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
  CheckoutSession, CheckoutSessionPaymentStatus, CheckoutSessionStatus, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionPaymentMethodTypes, CustomerId, IdOrCreate, ListCheckoutSessions, ListCheckoutSessionsCustomerDetails, ListPrices, ListSubscriptionItems, ListSubscriptions, Price, Subscription, SubscriptionId
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
    // http://localhost:5170/verify?success=true&session_id=cs_test_a1Owo7ibMsTCOjMWA0kMwUr8jAVqE539zKHPOCt7KtBK5oCb5XgQzW4xh7 
    //    log::info!("checkout session data : {:#?}", matched_session.unwrap());
    
    


//  subscription data

//    // Example of using ListProducts with specific parameters
//    let list_subscription_params = ListSubscriptions {
    //     customer:Some("cus_R2wKbdOIQC2vnI".to_string()),
    //     ..Default::default() // Fill in other fields as default
// };
// sub_1QAqRtRtqMxXmkr4l4tkiEZC

let subscription_data = Subscription::retrieve(&client, &"sub_1QAqRtRtqMxXmkr4l4tkiEZC".parse().unwrap(), &[]).await.unwrap();

   log::info!("subscription data : {:#?}", subscription_data);




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
        StatusCode::BAD_REQUEST,
        Json(json!({ "success": false, "message": "Plan verification unsuccessful" })),
    )
}

// id: CheckoutSessionId("cs_test_a1Q5f25QPXh6Bucg1rsSIVvqRcdEgjFbu0SohrTWYIs5SYlRJI4jbwuc2p")
// amount_total: Some(9999,  ),
//     cancel_url: Some("http://localhost:5170/verify?success=false&session_id={CHECKOUT_SESSION_ID}",),
//     created: 1728998391,
//     currency: Some(USD,
//     ),
//     customer: Some(Id(CustomerId("cus_R2F9C0q8VotyO2",
//     ),
//     customer_details: Some(PaymentPagesCheckoutSessionCustomerDetails{
//       address: Some(Address{
//         city: None,
//         country: Some("IN",
//         ),
//         line1: None,
//         line2: None,
//         postal_code: None,
//         state: None,

//       },
//       ),
//       email: Some("user0@gmail.com",
//       ),
//       name: Some("User2",
//       ),
//       phone: None,
//       tax_exempt: Some(None,
//       ),
//       tax_ids: Some([

//       ],
//       ),

//     },
//     ),
//     customer_email: Some("user0@gmail.com",
//     ),
//     expires_at: 1729084791,
//     invoice: Some(Id(InvoiceId("in_1QAAfQRtqMxXmkr406dPjAA9",
//     ),
//     mode: Subscription,

// subscription: Some(Id(SubscriptionId("sub_1QAAfQRtqMxXmkr4RO9vImxf",
//     ),
//     payment_method_options: Some(CheckoutSessionPaymentMethodOptions{
//       acss_debit: None,
//       affirm: None,
//       afterpay_clearpay: None,
//       alipay: None,
//       au_becs_debit: None,
//       bacs_debit: None,
//       bancontact: None,
//       boleto: None,
//       card: Some(CheckoutCardPaymentMethodOptions{
//         installments: None,
//         setup_future_usage: None,
//         statement_descriptor_suffix_kana: None,
//         statement_descriptor_suffix_kanji: None,

//       },
//       ),
//       cashapp: None,
//       customer_balance: None,
//       eps: None,
//       fpx: None,
//       giropay: None,
//       grabpay: None,
//       ideal: None,
//       klarna: None,
//       konbini: None,
//       link: None,
//       oxxo: None,
//       p24: None,
//       paynow: None,
//       paypal: None,
//       pix: None,
//       revolut_pay: None,
//       sepa_debit: None,
//       sofort: None,
//       swish: None,
//       us_bank_account: None,

//     },
//     ),
//     payment_method_types: [
//       "card",
//       "paypal",

//     ],
//     payment_status: Paid,
//     status: Some(Complete,
//     ),
//     success_url: Some("http://localhost:5170/verify?success=true&session_id={CHECKOUT_SESSION_ID}",
//     ),

//-------------------------------------------------------------------------------------------------------------------
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
//                             Id(
//                                 ProductId(
//                                     "prod_R1odaLLB9vHBxh",
//                                 ),
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
