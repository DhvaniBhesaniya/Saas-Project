use crate::configration::gett;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
// use std::env;

// Define the user model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDB ObjectId, auto-generated

    pub name: String,              // User's name
    pub email: String,             // User's email
    pub username: String,          // User's Username
    pub password: Option<String>,  // Hashed password for simple login
    pub google_id: Option<String>, // Google ID for Google login
    pub login_type: String,        // "google" or "email"

    pub subscription_id: Option<String>, // Plan details
    pub usage: Usage,                      // User's usage (tries)

    pub activity_log: Vec<ActivityLog>, // Activity log of user actions
    pub billing_history: Vec<BillingHistory>, // Billing history for the user
    pub address: CustomerDetails, // Customer details

    pub created_at: DateTime<Utc>, // Account creation timestamp
    pub updated_at: DateTime<Utc>, // Last update timestamp
    #[serde(rename = "profileImg")]
    pub profile_img: Option<String>,
    #[serde(rename = "AccDeleted")]
    pub acc_deleted: bool,
}

// Define the usage model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub tries_used: i32, // Number of tries used
    pub max_tries: i32,  // Max tries allowed
}

// Define the activity log model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityLog {
    pub event: String, // Description of the activity, e.g., "Login from Chrome on Windows"
    pub timestamp: DateTime<Utc>, // Timestamp of the activity
}

// Define the billing history model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingHistory {
    pub invoice_id: String,     // Invoice number, e.g., "Invoice #1234"
    pub paid_at: DateTime<Utc>, // Payment date and time
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

impl User {
    pub async fn get_user_collection() -> Collection<Document> {
        let url = gett::<String>("mongodb_url");
        let client_options = ClientOptions::parse(url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client.database("saas-data").collection::<Document>("users")
    }
}
