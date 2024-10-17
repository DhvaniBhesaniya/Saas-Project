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
    pub user_id: ObjectId,              // User ID linked to the subscription
    pub stripe_customer_id: String,     // Stripe customer ID
    pub stripe_customer_details: CustomerDetails, // Customer details
    pub plan_details: PlanDetails,      // Plan details (now a separate struct)
    pub auto_renew: bool,               // Whether the subscription auto-renews
    pub status: String,                 // "active", "canceled", or "expired"
    pub cancellation_date: Option<DateTime<Utc>>, // Set if the subscription is canceled
    pub payment_method: String,         // Payment method used, e.g., Stripe, PayPal
    pub payment_history: Vec<PaymentDetails>, // Array of payment history records
    pub last_payment_date: Option<DateTime<Utc>>, // Date of the last payment
    pub next_billing_date: Option<DateTime<Utc>>, // Date of the next billing cycle
}

// Define the plan details struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanDetails {
    pub plan_type: String,         // "monthly", "yearly", or custom plan name
    pub billing_cycle: String,     // Billing cycle, e.g., "monthly" or "yearly"
    pub start_date: DateTime<Utc>, // Subscription start date
    pub end_date: DateTime<Utc>,   // Subscription end date
}

// Define the customer details struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerDetails {
    pub address: Option<Address>,     // Customer address
    pub email: Option<String>,        // Customer email
    pub name: Option<String>,         // Customer name
    pub phone: Option<String>,        // Customer phone
    pub tax_exempt: Option<bool>,     // Tax exemption status
    pub tax_ids: Option<Vec<String>>, // Tax IDs if applicable
}

// Define the address struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub city: Option<String>,        // City
    pub country: Option<String>,     // Country
    pub line1: Option<String>,       // Address line 1
    pub line2: Option<String>,       // Address line 2
    pub postal_code: Option<String>, // Postal code
    pub state: Option<String>,       // State or province
}

// Define the payment details struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentDetails {
    pub invoice_id: String,  // Invoice number, e.g., "Invoice #1234"
    pub amount: f64,         // Payment amount
    pub date: DateTime<Utc>, // Date of payment
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
