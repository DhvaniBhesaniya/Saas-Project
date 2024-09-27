// use crate::configration::gett;
// use mongodb::{options::ClientOptions, Client};

// pub async fn connect_db() -> Client {
//     let url = gett::<String>("mongodb_url");
//     let client_options = ClientOptions::parse(url).await.unwrap();
//     let client = Client::with_options(client_options).unwrap();
//     println!("DB connected");
//     client
// }

use crate::configration::gett;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

pub async fn connect_db() -> Client {
    // Get MongoDB connection URL from configuration
    let url = gett::<String>("mongodb_url");

    // Parse client options with the connection string
    let mut client_options = ClientOptions::parse(&url).await.unwrap();

    // Set the server_api field to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new MongoDB client with the given options
    let client = Client::with_options(client_options).unwrap();

    // Send a ping to confirm the connection to the MongoDB server
    let ping_result = client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await;

    match ping_result {
        Ok(_) => {
            // println!("Pinged your deployment. You successfully connected to MongoDB!");
            log::info!("Pinged your deployment. You successfully connected to MongoDB!")
        }
        Err(e) => {
            // println!("Failed to ping MongoDB: {:?}", e);
            log::error!("Failed to ping MongoDB: {:?}", e);
        }
    }

    // Return the client instance after successful connection
    client
}
