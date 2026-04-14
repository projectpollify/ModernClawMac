use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{Duration, Instant};

use crate::types::{ChatMessage, ChatResponse, Model, ModelDetails, OllamaStatus};

const LM_STUDIO_BASE_URL: &str = "http://127.0.0.1:1234/v1";

pub struct LMStudioService {
    client: Client,
    base_url: String,
}

impl LMStudioService {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .expect("failed to create LM Studio HTTP client");

        Self {
            client,
            base_url: LM_STUDIO_BASE_URL.to_string(),
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
                version: Some("LM Studio".to_string()),
                error: None,
            },
            Ok(response) => OllamaStatus {
                running: false,
                version: Some("LM Studio".to_string()),
                error: Some(format!("Unexpected status: {}", response.status())),
            },
            Err(error) => OllamaStatus {
                running: false,
                version: Some("LM Studio".to_string()),
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

        let payload: LMStudioModelsResponse = response
            .json()
            .await
            .map_err(|error| format!("Parse error: {}", error))?;

        Ok(payload
            .data
            .into_iter()
            .filter(|model| is_chat_model(&model.id))
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
        let request = LMStudioChatRequest {
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

        let payload: LMStudioChatCompletionResponse = response
            .json()
            .await
            .map_err(|error| format!("Parse error: {}", error))?;

        let content = payload
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| "LM Studio returned no assistant message.".to_string())?;
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
            "ModernClawMac reads loaded models from LM Studio. Load {} inside LM Studio, then refresh the model list here.",
            name
        ))
    }

    pub async fn delete_model(&self, name: &str) -> Result<(), String> {
        Err(format!(
            "ModernClawMac does not delete models from LM Studio directly. Unload or remove {} inside LM Studio.",
            name
        ))
    }
}

#[derive(Debug, Deserialize)]
struct LMStudioModelsResponse {
    data: Vec<LMStudioModel>,
}

#[derive(Debug, Deserialize)]
struct LMStudioModel {
    id: String,
}

#[derive(Debug, Serialize)]
struct LMStudioChatRequest {
    model: String,
    messages: Vec<LMStudioChatMessage>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct LMStudioChatMessage {
    role: String,
    content: LMStudioMessageContent,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum LMStudioMessageContent {
    Text(String),
    Parts(Vec<LMStudioContentPart>),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum LMStudioContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: LMStudioImageUrl },
}

#[derive(Debug, Serialize)]
struct LMStudioImageUrl {
    url: String,
}

#[derive(Debug, Deserialize)]
struct LMStudioChatCompletionResponse {
    created: Option<i64>,
    model: String,
    choices: Vec<LMStudioChoice>,
    usage: Option<LMStudioUsage>,
}

#[derive(Debug, Deserialize)]
struct LMStudioChoice {
    message: LMStudioAssistantMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LMStudioAssistantMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LMStudioUsage {
    prompt_tokens: Option<i64>,
    completion_tokens: Option<i64>,
}

fn prepare_message(message: ChatMessage) -> Result<LMStudioChatMessage, String> {
    if message.images.is_empty() {
        return Ok(LMStudioChatMessage {
            role: message.role,
            content: LMStudioMessageContent::Text(message.content),
        });
    }

    let mut parts = Vec::new();

    if !message.content.trim().is_empty() {
        parts.push(LMStudioContentPart::Text { text: message.content });
    }

    for path in message.images {
        let bytes = std::fs::read(&path).map_err(|error| format!("Failed to read image {}: {}", path, error))?;
        let mime = infer_mime_type(&path);
        parts.push(LMStudioContentPart::ImageUrl {
            image_url: LMStudioImageUrl {
                url: format!("data:{};base64,{}", mime, STANDARD.encode(bytes)),
            },
        });
    }

    Ok(LMStudioChatMessage {
        role: message.role,
        content: LMStudioMessageContent::Parts(parts),
    })
}

fn infer_family(lower_name: &str) -> Option<String> {
    if lower_name.contains("embed") || lower_name.contains("embedding") {
        return Some("embedding".to_string());
    }

    if lower_name.contains("gemma4") || lower_name.contains("gemma-4") {
        return Some("gemma4".to_string());
    }

    if lower_name.contains("gemma") {
        return Some("gemma".to_string());
    }

    if lower_name.contains("llama") {
        return Some("llama".to_string());
    }

    if lower_name.contains("qwen") {
        return Some("qwen".to_string());
    }

    None
}

fn is_chat_model(model_name: &str) -> bool {
    let lower = model_name.to_lowercase();
    !(lower.contains("embed") || lower.contains("embedding"))
}

fn infer_mime_type(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "image/png",
    }
}
