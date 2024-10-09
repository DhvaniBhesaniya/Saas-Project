use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;
use std::collections::VecDeque;

lazy_static::lazy_static! {
    // A simple in-memory store for chat history
    static ref CHAT_HISTORY: Mutex<VecDeque<(String, String)>> = Mutex::new(VecDeque::new());
}



const MODEL_GEMINI: &str = "gemini-1.5-flash-latest";
// export GEMINI_API_KEY=AIzaSyBQ5M1hU84zqMNYv4EOFihMQTRLrZ-ii5I

// Struct to handle the input request
#[derive(Deserialize)]
pub struct TextaiRequest {
    pub text: String,
    pub language: String,
}

// Struct to handle the response that will be returned as JSON
#[derive(Serialize)]
pub struct TextaiResponse {
    pub data: Option<String>,
    pub message: String,
}
#[derive(Deserialize)]
pub struct ChataiRequest {
    pub textmessage: String,
    pub language: String,
}

// Struct to handle the response that will be returned as JSON
#[derive(Serialize)]
pub struct ChataiResponse {
    pub data: Option<String>,
    pub message: String,
}

pub async fn genaitext(Json(payload): Json<TextaiRequest>) -> Response {
    // Extract text and target language from the payload
    let text_to_translate = payload.text;
    let target_language = payload.language;

    // Create the chat request with the provided text and target language
    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("Translate the following text to the target language."),
        ChatMessage::user(&format!(
            "Text: {}\nTarget Language: {}",
            text_to_translate, target_language
        )),
    ]);

    // Create the GenAI client
    let client = Client::default();

    // Check if the GEMINI_API_KEY environment variable is set
    if env::var("GEMINI_API_KEY").is_err() {
        let response = TextaiResponse {
            data: None,
            message: "GEMINI_API_KEY environment variable not set".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    // Resolve model information
    let _adapter_kind = match client.resolve_model_iden(MODEL_GEMINI) {
        Ok(info) => info.adapter_kind,
        Err(e) => {
            let response = TextaiResponse {
                data: None,
                message: format!("Error resolving model info: {}", e),
            };
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // log::info!("\n===== MODEL: {} ({}) =====", MODEL_GEMINI, adapter_kind);
    // log::info!("\n--- Text to Translate:\n{}", text_to_translate);

    // Execute chat request for translation
    match client.exec_chat(MODEL_GEMINI, chat_req.clone(), None).await {
        Ok(chat_res) => {
            // Get the translated text from the response
            let translated_text = chat_res.content_text_as_str().unwrap_or("NO ANSWER");

            // log::info!("\n--- Translated Text:\n{}", translated_text);

            // Return successful response with translated text
            let response = TextaiResponse {
                data: Some(translated_text.to_string()),
                message: "Translation successful".to_string(),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            // Return error response
            let response = TextaiResponse {
                data: None,
                message: format!("Error during translation: {}", e),
            };
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

pub async fn genaidoc() {}
pub async fn genaichat(Json(payload): Json<ChataiRequest>) -> Response {
    // Extract text message and target language from the payload
    let text_message = payload.textmessage;
    let target_language = payload.language;

    // Create the chat request with the provided message and target language
    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("You are an  knowledgable ai,  you will be getting a  user message in  text :  ... and Target Language: ... , 
        what you need to do is give the response of  what the normally you give to the user who asks you any thing, consider  the user text as an question so give the answers according to it , the Target Language is given because  
        you need to send your response in that perticular language , thats why it is given. "),
        ChatMessage::user(&format!(
            "Text: {}\nTarget Language: {}",
            text_message, target_language
        )),
    ]);

    // Create the GenAI client
    let client = Client::default();

    // Check if the GEMINI_API_KEY environment variable is set
    if env::var("GEMINI_API_KEY").is_err() {
        let response = ChataiResponse {
            data: None,
            message: "GEMINI_API_KEY environment variable not set".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    // Resolve model information
    let _adapter_kind = match client.resolve_model_iden(MODEL_GEMINI) {
        Ok(info) => info.adapter_kind,
        Err(e) => {
            let response = ChataiResponse {
                data: None,
                message: format!("Error resolving model info: {}", e),
            };
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // Execute chat request for AI response
    match client.exec_chat(MODEL_GEMINI, chat_req.clone(), None).await {
        Ok(chat_res) => {
            // Get the AI response from the result
            let ai_response = chat_res.content_text_as_str().unwrap_or("NO ANSWER");

            // Return successful response with AI-generated chat
            let response = ChataiResponse {
                data: Some(ai_response.to_string()),
                message: "Chat successful".to_string(),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            // Return error response if the AI chat fails
            let response = ChataiResponse {
                data: None,
                message: format!("Error during AI chat: {}", e),
            };
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

