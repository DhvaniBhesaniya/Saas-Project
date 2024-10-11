use std::env;

use crate::configration::gett;

pub async fn set_env_variable() -> Result<String, String> {
    let gemini_api_key: String = gett("GEMINI_API_KEY");
    
    match env::var("GEMINI_API_KEY") {
        Ok(_) => {
            log::info!("GEMINI_API_KEY is already set");
            Ok("GEMINI_API_KEY is already set".to_string())
        }
        Err(_) => {
            env::set_var("GEMINI_API_KEY", gemini_api_key); // Set the environment variable
            log::info!("Successfully set the env variable.");
            Ok("Successfully set the env variable.".to_string())
        }
    }
}
