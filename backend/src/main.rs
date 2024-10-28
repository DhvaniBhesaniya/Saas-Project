use axum::{routing::get, Router};
// use chrono::{FixedOffset, TimeZone, Utc};
use middleware::cors::cors_layer;
use routes::genai_routes::create_genai_routes;
use routes::subscription_routes::create_subscription_routes;
use tower::ServiceBuilder;
use utils::db::connect_db;
use utils::set_env::set_env_variable;

use crate::routes::user_route::create_user_routes;
use std::net::SocketAddr;

use tokio;

mod configration;
mod controllers;
mod middleware;
mod models;
mod routes;
mod utils;

use crate::configration::gett;
use crate::utils::logger;
#[tokio::main]
async fn main() {
    // Initialize Logger
    logger::startLogger();
    let _db_client = connect_db().await;
    let _set_env = set_env_variable().await;

    let app = Router::new()
        .route("/test", get(handler))
        .merge(create_user_routes())
        .merge(create_genai_routes())
        .merge(create_subscription_routes())
        .layer(ServiceBuilder::new().layer(cors_layer()));

    let addr = SocketAddr::from(([127, 0, 0, 1], gett("port")));

    println!("Server running on http://{}", addr);
    // timestamp_to_bson_with_string().await;

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    log::info!("Rust API working");
    "Saas API working.."
}

// If you need both BsonDateTime and the formatted string:  1729159014
// async fn timestamp_to_bson_with_string() {
//     // println!("Input timestamp: {}", 1727675860 as i64);

//     // let bson_dt = BsonDateTime::from_millis(1727675860 as i64 * 1000);
//     // println!("BsonDateTime raw: {:?}", bson_dt);

//     // let utc_time = Utc.timestamp_opt(1727675860 as i64, 0).unwrap();
//     // println!("UTC Time: {:?}", utc_time);

//     // utc now
//     let utc_time = Utc::now();
//     println!("UTC Time: {:?}", utc_time);

//     // let formatted_string = utc_time
//     //     .format("%Y-%m-%d %H:%M:%S.000 +00:00:00")
//     //     .to_string();

//     // Create an offset for UTC+5:30 (India)
//     let india_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
//     let local_time = utc_time.with_timezone(&india_offset);

//     // Format the local time with AM/PM
//     let formatted_string = local_time.format("%Y-%m-%d %I:%M:%S %p").to_string();
//     println!("Indian time: Formatted string: {}", formatted_string);
// }

// output:
// Input timestamp: 1729159014
// BsonDateTime raw: DateTime(2024-10-17 9:56:54.0 +00:00:00)
// UTC Time: 2024-10-17T09:56:54Z
// Formatted string: 2024-10-17 09:56:54.000 +00:00:00

// why still 9:56:54

// i said i want my time here which is 3:30:14 not  this 09:56:54

// or is there any method in the node

// like if i am getting this

// const parsedDate = new Date(cleanedDateStr);
//    // 2024-09-30T05:57:00.000Z
// // this above is same bason utc time  which i am extracting fom by db so it is looking like this
// can we convert this to like this
// 2023-07-01 10:30 AM format    this is a sample format but i need  fromt he given time to convert it to this fromat in node.
