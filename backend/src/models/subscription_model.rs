use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use crate::configration::gett;

// Define the subscription plan model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionPlan {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDB ObjectId, auto-generated
    pub stripe_subscription_id: String, // Stripe subscription ID
    pub user_id: String,              // User ID linked to the subscription
    pub stripe_customer_id: String,     // Stripe customer ID
    pub plan_details: PlanDetails,      // Plan details (now a separate struct)
    pub auto_renew: bool,               // Whether the subscription auto-renews
    pub refundable: bool,
    pub status: String,                 // "active", "canceled", or "expired"
    pub cancellation_date: Option<DateTime<Utc>>, // Set if the subscription is canceled
    pub payment_history: Vec<PaymentDetails>, // Array of payment history records
}

// Define the plan details struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanDetails {
    pub plan_id: String,
    pub product_id: String,
    pub plan_name: String,         // custom plan name
    pub billing_cycle: String,     // Billing cycle, e.g., "monthly" or "yearly"
    pub start_date: String, // Subscription start date
    pub end_date: String,   // Subscription end date
}

// Define the payment details struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentDetails {
    pub invoice_id: String,     // Invoice number, e.g., "Invoice #1234"
    pub invoice_pdf: String,
    pub payment_method: String, // Payment method used, e.g., Stripe, PayPal
    pub currency: String,
    pub amount: f64,         // Payment amount
    pub payment_date: String, // Payment date
}

// Implement methods for accessing the MongoDB collection
impl SubscriptionPlan {
    pub async fn get_subscription_collection() -> Collection<Document> {
        let url = gett::<String>("mongodb_url");
        let client_options = ClientOptions::parse(url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client
            .database("saas-data")
            .collection::<Document>("subscriptions")
    }
}
