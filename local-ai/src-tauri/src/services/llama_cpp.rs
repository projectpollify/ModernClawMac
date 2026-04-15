use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{Duration, Instant};

use crate::types::{ChatMessage, ChatResponse, Model, ModelDetails, OllamaStatus};

const LLAMA_CPP_BASE_URL: &str = "http://127.0.0.1:8080/v1";

pub struct LlamaCppService {
    client: Client,
    base_url: String,
}

impl LlamaCppService {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .expect("failed to create llama.cpp HTTP client");

        Self {
            client,
            base_url: LLAMA_CPP_BASE_URL.to_string(),
        }
    }

    pub async fn check_status(&self) -> OllamaStatus {
        match self
            .client
            .get(format!("{}/models", self.base_url))
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => OllamaStatus {
                running: true,
                version: Some("llama.cpp".to_string()),
                error: None,
            },
            Ok(response) => OllamaStatus {
                running: false,
                version: Some("llama.cpp".to_string()),
                error: Some(format!("Unexpected status: {}", response.status())),
            },
            Err(error) => OllamaStatus {
                running: false,
                version: Some("llama.cpp".to_string()),
                error: Some(error.to_string()),
            },
        }
    }

    pub async fn list_models(&self) -> Result<Vec<Model>, String> {
        let response = self
            .client
            .get(format!("{}/models", self.base_url))
            .send()
            .await
            .map_err(|error| format!("Request failed: {}", error))?;

        if !response.status().is_success() {
            return Err(format!("Failed to list models: {}", response.status()));
        }

        let payload: LlamaCppModelsResponse = response
            .json()
            .await
            .map_err(|error| format!("Parse error: {}", error))?;

        Ok(payload
            .data
            .into_iter()
            .map(|model| {
                let normalized_name = model.id.clone();
                let lower = normalized_name.to_lowercase();

                Model {
                    name: normalized_name.clone(),
                    modified_at: String::new(),
                    size: 0,
                    digest: normalized_name,
                    details: ModelDetails {
                        format: None,
                        family: infer_family(&lower),
                        parameter_size: None,
                        quantization_level: None,
                    },
                }
            })
            .collect())
    }

    pub async fn chat_stream<F>(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        mut on_chunk: F,
    ) -> Result<(), String>
    where
        F: FnMut(ChatResponse) + Send,
    {
        let started_at = Instant::now();
        let request = LlamaCppChatRequest {
            model: model.to_string(),
            messages: messages
                .into_iter()
                .map(prepare_message)
                .collect::<Result<Vec<_>, _>>()?,
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|error| format!("Request failed: {}", error))?;

        let status = response.status();

        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            if error_body.trim().is_empty() {
                return Err(format!("Chat failed: {}", status));
            }

            return Err(format!("Chat failed: {} - {}", status, error_body));
        }

        let payload: LlamaCppChatCompletionResponse = response
            .json()
            .await
            .map_err(|error| format!("Parse error: {}", error))?;

        let content = payload
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| "llama.cpp returned no assistant message.".to_string())?;
        let finish_reason = payload
            .choices
            .first()
            .and_then(|choice| choice.finish_reason.clone());

        let created_at = payload
            .created
            .and_then(|timestamp| Utc.timestamp_opt(timestamp, 0).single())
            .unwrap_or_else(Utc::now)
            .to_rfc3339();

        on_chunk(ChatResponse {
            model: payload.model,
            created_at,
            message: ChatMessage {
                role: "assistant".to_string(),
                content,
                images: Vec::new(),
            },
            done: true,
            total_duration: u64::try_from(started_at.elapsed().as_nanos()).ok(),
            eval_count: payload
                .usage
                .as_ref()
                .and_then(|usage| usage.completion_tokens.and_then(|count| u32::try_from(count).ok())),
            prompt_eval_count: payload
                .usage
                .as_ref()
                .and_then(|usage| usage.prompt_tokens.and_then(|count| u32::try_from(count).ok())),
            finish_reason,
        });

        Ok(())
    }

    pub async fn pull_model(&self, name: &str) -> Result<(), String> {
        Err(format!(
            "Direct-engine model install is not implemented yet. Add or configure {} for llama.cpp, then refresh the model list.",
            name
        ))
    }

    pub async fn delete_model(&self, name: &str) -> Result<(), String> {
        Err(format!(
            "Direct-engine model deletion is not implemented yet. Remove or unload {} from the llama.cpp setup manually.",
            name
        ))
    }
}

#[derive(Debug, Deserialize)]
struct LlamaCppModelsResponse {
    data: Vec<LlamaCppModel>,
}

#[derive(Debug, Deserialize)]
struct LlamaCppModel {
    id: String,
}

#[derive(Debug, Serialize)]
struct LlamaCppChatRequest {
    model: String,
    messages: Vec<LlamaCppChatMessage>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct LlamaCppChatMessage {
    role: String,
    content: LlamaCppMessageContent,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum LlamaCppMessageContent {
    Text(String),
    Parts(Vec<LlamaCppContentPart>),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum LlamaCppContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: LlamaCppImageUrl },
}

#[derive(Debug, Serialize)]
struct LlamaCppImageUrl {
    url: String,
}

#[derive(Debug, Deserialize)]
struct LlamaCppChatCompletionResponse {
    model: String,
    #[serde(default)]
    created: Option<i64>,
    choices: Vec<LlamaCppChatChoice>,
    #[serde(default)]
    usage: Option<LlamaCppUsage>,
}

#[derive(Debug, Deserialize)]
struct LlamaCppChatChoice {
    message: LlamaCppAssistantMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LlamaCppAssistantMessage {
    #[serde(default)]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LlamaCppUsage {
    #[serde(default)]
    prompt_tokens: Option<i64>,
    #[serde(default)]
    completion_tokens: Option<i64>,
}

fn prepare_message(message: ChatMessage) -> Result<LlamaCppChatMessage, String> {
    let ChatMessage {
        role,
        content,
        images,
    } = message;

    if images.is_empty() {
        return Ok(LlamaCppChatMessage {
            role,
            content: LlamaCppMessageContent::Text(content),
        });
    }

    let mut parts = vec![LlamaCppContentPart::Text { text: content }];

    for image_path in images {
        let image_bytes = std::fs::read(&image_path)
            .map_err(|error| format!("Failed to read image {}: {}", image_path, error))?;
        let mime_type = mime_type_for_path(&image_path);
        let encoded = STANDARD.encode(image_bytes);
        parts.push(LlamaCppContentPart::ImageUrl {
            image_url: LlamaCppImageUrl {
                url: format!("data:{};base64,{}", mime_type, encoded),
            },
        });
    }

    Ok(LlamaCppChatMessage {
        role,
        content: LlamaCppMessageContent::Parts(parts),
    })
}

fn mime_type_for_path(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    }
}

fn infer_family(lower_name: &str) -> Option<String> {
    if lower_name.contains("gemma") {
        Some("gemma".to_string())
    } else if lower_name.contains("llama") {
        Some("llama".to_string())
    } else if lower_name.contains("qwen") {
        Some("qwen".to_string())
    } else if lower_name.contains("mistral") {
        Some("mistral".to_string())
    } else {
        None
    }
}
