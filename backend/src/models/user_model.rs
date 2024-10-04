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

    pub subscription_plan: SubscriptionPlan, // Plan details
    pub usage: Usage,                        // User's usage (tries)

    pub created_at: DateTime<Utc>, // Account creation timestamp
    pub updated_at: DateTime<Utc>, // Last update timestamp
    #[serde(rename = "profileImg")]
    pub profile_img: Option<String>,
    #[serde(rename = "AccDeleted")]
    pub acc_deleted: bool,
}

// Define the subscription plan model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionPlan {
    pub plan_type: String,                 // "free", "custom", "recommended"
    pub start_date: Option<DateTime<Utc>>, // Subscription start date
    pub end_date: Option<DateTime<Utc>>,   // Subscription end date
    pub payment_status: Option<String>,    // "active", "pending", "canceled"
}

// Define the usage model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub tries_used: i32, // Number of tries used
    pub max_tries: i32,  // Max tries allowed
}

impl User {
    pub async fn get_user_collection() -> Collection<Document> {
        let url = gett::<String>("mongodb_url");
        let client_options = ClientOptions::parse(url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client.database("saas-data").collection::<Document>("users")
    }
}
