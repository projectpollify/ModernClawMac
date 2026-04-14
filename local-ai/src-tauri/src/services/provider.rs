use crate::types::{ChatMessage, ChatResponse, Model, OllamaStatus};

use super::lm_studio::LMStudioService;
use super::ollama::OllamaService;

pub enum ProviderService {
    Ollama(OllamaService),
    LMStudio(LMStudioService),
}

impl ProviderService {
    pub fn new_default() -> Self {
        if cfg!(target_os = "macos") {
            Self::LMStudio(LMStudioService::new())
        } else {
            Self::Ollama(OllamaService::new())
        }
    }

    pub async fn check_status(&self) -> OllamaStatus {
        match self {
            Self::Ollama(service) => service.check_status().await,
            Self::LMStudio(service) => service.check_status().await,
        }
    }

    pub async fn list_models(&self) -> Result<Vec<Model>, String> {
        match self {
            Self::Ollama(service) => service.list_models().await,
            Self::LMStudio(service) => service.list_models().await,
        }
    }

    pub async fn chat_stream<F>(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        on_chunk: F,
    ) -> Result<(), String>
    where
        F: FnMut(ChatResponse) + Send,
    {
        match self {
            Self::Ollama(service) => service.chat_stream(model, messages, None, on_chunk).await,
            Self::LMStudio(service) => service.chat_stream(model, messages, on_chunk).await,
        }
    }

    pub async fn pull_model(&self, name: &str) -> Result<(), String> {
        match self {
            Self::Ollama(service) => service.pull_model(name).await,
            Self::LMStudio(service) => service.pull_model(name).await,
        }
    }

    pub async fn delete_model(&self, name: &str) -> Result<(), String> {
        match self {
            Self::Ollama(service) => service.delete_model(name).await,
            Self::LMStudio(service) => service.delete_model(name).await,
        }
    }
}
