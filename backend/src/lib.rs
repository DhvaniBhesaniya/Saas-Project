
// use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {
//   let mut client_options =
//     ClientOptions::parse("mongodb+srv://narutogokuluffy009:<db_password>@zoro.1a4dm.mongodb.net/?retryWrites=true&w=majority&appName=Zoro")
//       $.await?;

//   // Set the server_api field of the client_options object to set the version of the Stable API on the client
//   let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//   client_options.server_api = Some(server_api);

//   // Get a handle to the cluster
//   let client = Client::with_options(client_options)?;

//   // Ping the server to see if you can connect to the cluster
//   client
//     .database("admin")
//     .run_command(doc! {"ping": 1}, None)
//     .await?;
//   println!("Pinged your deployment. You successfully connected to MongoDB!");

//   Ok(())
// }





// https://stackoverflow.com/questions/73041173/is-it-possible-to-use-env-file-at-build-time
// for dot.env   , https://crates.io/crates/dotenv_codegen







// I'm not so sure this is well-established, but you can use a build script that will read the file and use println!("cargo:rustc-env=VAR=VALUE") to send the environment variables to Cargo, allowing you to retrieve them in the code with env!() or option_env!().

// For example, to use a .env file, add dotenv to build-dependencies, and use it like so in build.rs:

// fn main() {
//     let dotenv_path = dotenv::dotenv().expect("failed to find .env file");
//     println!("cargo:rerun-if-changed={}", dotenv_path.display());

//     // Warning: `dotenv_iter()` is deprecated! Roll your own or use a maintained fork such as `dotenvy`.
//     for env_var in dotenv::dotenv_iter().unwrap() {
//         let (key, value) = env_var.unwrap();
//         println!("cargo:rustc-env={key}={value}");
//     }
// }