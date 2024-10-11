use axum::extract::Multipart;
use axum::{http::header, http::StatusCode, response::IntoResponse, response::Response, Json};
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
// use std::fs::{create_dir_all, File};
// use std::io::Write;
// use std::path::Path;
use std::env;
// use std::sync::Mutex;
// use std::collections::VecDeque;

// lazy_static::lazy_static! {
//     // A simple in-memory store for chat history
//     static ref CHAT_HISTORY: Mutex<VecDeque<(String, String)>> = Mutex::new(VecDeque::new());
// }

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

// Original genaitext function can now use the helper
pub async fn genaitext(Json(payload): Json<TextaiRequest>) -> impl IntoResponse {
    match translate_text(payload.text, payload.language).await {
        Ok(translated_text) => {
            let response = TextaiResponse {
                data: Some(translated_text),
                message: "Translation successful".to_string(),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            let response = TextaiResponse {
                data: None,
                message: err,
            };
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

pub async fn genaidoc(mut multipart: Multipart) -> impl IntoResponse {
    let mut target_language = None;
    let mut original_file_name = None;
    let mut file_content = String::new();
    let max_size = 10 * 1024 * 1024; // 10 MB max

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "language" {
            target_language = Some(field.text().await.unwrap());
        } else if name == "file" {
            original_file_name = field.file_name().map(|s| s.to_string());
            let data = field.bytes().await.unwrap();

            if data.len() > max_size {
                return (StatusCode::BAD_REQUEST, "File too large".to_string()).into_response();
            }

            // Ensure the file is either .txt or .doc
            if let Some(ref file_name) = original_file_name {
                if !file_name.ends_with(".txt") && !file_name.ends_with(".doc") {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "success": false,
                            "message": "Unsupported file type"
                        })),
                    )
                        .into_response();
                }
            }

            file_content = String::from_utf8(data.to_vec()).unwrap();
        }
    }

    // Ensure the language is provided
    let language = match target_language {
        Some(lang) => lang,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "message": "Target language not specified"
                })),
            )
                .into_response()
        }
    };

    // Translate the content using the same logic as in genaitext
    match translate_text(file_content.clone(), language.clone()).await {
        Ok(translated_content) => {
            // Set up the file name
            let file_name = if let Some(file_name) = original_file_name.clone() {
                // Modify file name to include language suffix
                if file_name.ends_with(".txt") {
                    file_name.replace(".txt", &format!("_translated_{}.txt", language))
                } else if file_name.ends_with(".doc") {
                    file_name.replace(".doc", &format!("_translated_{}.doc", language))
                } else {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "success": false,
                            "message": "Unsupported file type"
                        })),
                    )
                        .into_response();
                }
            } else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "success": false,
                        "message": "File name not provided"
                    })),
                )
                    .into_response();
            };

            // Set the content-disposition header to trigger file download
            let content_disposition = format!("attachment; filename=\"{}\"", file_name);

            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/octet-stream"),
                    (header::CONTENT_DISPOSITION, &content_disposition),
                ],
                translated_content.into_bytes(),
            )
                .into_response()
            // (
            //     StatusCode::OK,
            //     [
            //         (header::CONTENT_TYPE, "application/octet-stream"),
            //         (header::CONTENT_DISPOSITION, &content_disposition),
            //     ],
            //     Json(json!({
            //         "success": true,
            //         "data": translated_content.into_bytes()
            //     })),
                
            // ).into_response()
        }
        Err(err) => (StatusCode::BAD_REQUEST, err).into_response(),
    }
}

// Helper function to translate text
async fn translate_text(
    text_to_translate: String,
    target_language: String,
) -> Result<String, String> {
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
        return Err("GEMINI_API_KEY environment variable not set".to_string());
    }

    // Resolve model information
    let _adapter_kind = match client.resolve_model_iden(MODEL_GEMINI) {
        Ok(info) => info.adapter_kind,
        Err(e) => return Err(format!("Error resolving model info: {}", e)),
    };

    // Execute chat request for translation
    match client.exec_chat(MODEL_GEMINI, chat_req.clone(), None).await {
        Ok(chat_res) => {
            // Get the translated text from the response
            let translated_text = chat_res.content_text_as_str().unwrap_or("NO ANSWER");
            Ok(translated_text.to_string())
        }
        Err(e) => Err(format!("Error during translation: {}", e)),
    }
}
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

// match translate_text(file_content.clone(), language.clone()).await {
//     /*************  ✨ Codeium Command 🌟  *************/
//             Ok(translated_content) => {
//                 // Set up the file name

//                 let dir_path = Path::new("./translated_documents");
//                 let output_file_name = if let Some(file_name) = original_file_name.clone() {
//                     // Modify file name to include language suffix
//                     if file_name.ends_with(".txt") {
//                         file_name.replace(".txt", &format!("_translated_{}.txt", language))
//                     } else if file_name.ends_with(".doc") {
//                         file_name.replace(".doc", &format!("_translated_{}.doc", language))
//                     } else {
//                         return (StatusCode::BAD_REQUEST, "Unsupported file type".to_string())
//                             .into_response();
//                     }
//                 } else {
//                     return (
//                         StatusCode::BAD_REQUEST,
//                         "File name not provided".to_string(),
//                     )
//                         .into_response();
//                 };

//                 let file_path = dir_path.join(&output_file_name);

//                 // Ensure the directory exists, create it if necessary
//                 if !dir_path.exists() {
//                     create_dir_all(dir_path).expect("Unable to create directory");
//                 }
//                  // Save the translated content to the file
//                  let mut file = File::create(&file_path).expect("Unable to create file");
//                  file.write_all(translated_content.as_bytes()).expect("Unable to write to file");

//                 let file_name = format!("{}", output_file_name);
//                 // log::info!("file name : {}",file_name);

//                 let file_path = format!("./translated_documents/{}", file_name);
//                 let file_bytes = fs::read(&file_path).expect("Unable to read the file");

//                 // Set the content-disposition header to trigger file download
//                 let content_disposition = format!("attachment; filename=\"{}\"", file_name);

//                 (
//                     StatusCode::OK,
//                     [
//                         (header::CONTENT_TYPE, "application/octet-stream"),
//                         (header::CONTENT_DISPOSITION, &content_disposition),
//                     ],

//                     file_bytes,
//                 )
//                     .into_response()
//             }

//             Err(err) => (StatusCode::BAD_REQUEST, err).into_response(),
//         }
