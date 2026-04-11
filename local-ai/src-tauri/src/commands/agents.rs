use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::agent_repo::{AgentRepository, JOE_SUPPORT_ID};
use crate::types::Agent;
use crate::{DatabaseState, MemoryState};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDto {
    pub agent_id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub workspace_path: Option<String>,
    #[serde(default)]
    pub default_model: Option<String>,
    #[serde(default)]
    pub enable_voice_output: Option<bool>,
    #[serde(default)]
    pub piper_voice_preset: Option<String>,
    #[serde(default)]
    pub piper_model_path: Option<String>,
    #[serde(default)]
    pub enable_voice_input: Option<bool>,
    #[serde(default)]
    pub whisper_model_path: Option<String>,
    #[serde(default)]
    pub whisper_language: Option<String>,
    #[serde(default)]
    pub profile_kind: Option<String>,
    #[serde(default)]
    pub shares_primary_workspace: Option<bool>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Agent> for AgentDto {
    fn from(value: Agent) -> Self {
        let profile_kind = if value.agent_id == JOE_SUPPORT_ID {
            "support"
        } else {
            "main"
        };

        Self {
            agent_id: value.agent_id,
            name: value.name,
            description: value.description,
            status: Some(value.status),
            workspace_path: Some(value.workspace_path),
            default_model: value.default_model,
            enable_voice_output: value.enable_voice_output,
            piper_voice_preset: value.piper_voice_preset,
            piper_model_path: value.piper_model_path,
            enable_voice_input: value.enable_voice_input,
            whisper_model_path: value.whisper_model_path,
            whisper_language: value.whisper_language,
            profile_kind: Some(profile_kind.to_string()),
            shares_primary_workspace: Some(true),
            created_at: Some(value.created_at),
            updated_at: Some(value.updated_at),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentVoiceSettingsDto {
    #[serde(default)]
    pub enable_voice_output: Option<bool>,
    #[serde(default)]
    pub piper_voice_preset: Option<String>,
    #[serde(default)]
    pub piper_model_path: Option<String>,
    #[serde(default)]
    pub enable_voice_input: Option<bool>,
    #[serde(default)]
    pub whisper_model_path: Option<String>,
    #[serde(default)]
    pub whisper_language: Option<String>,
}

#[tauri::command]
pub async fn agent_list(state: State<'_, DatabaseState>) -> Result<Vec<AgentDto>, String> {
    let repo = AgentRepository::new(&state.db);
    Ok(repo.list()?.into_iter().map(Into::into).collect())
}

#[tauri::command]
pub async fn agent_get_active(
    db_state: State<'_, DatabaseState>,
    memory_state: State<'_, MemoryState>,
) -> Result<AgentDto, String> {
    let repo = AgentRepository::new(&db_state.db);
    Ok(repo.get_active_agent(&memory_state.root_path)?.into())
}

#[tauri::command]
pub async fn agent_update_default_model(
    db_state: State<'_, DatabaseState>,
    memory_state: State<'_, MemoryState>,
    agent_id: String,
    default_model: Option<String>,
) -> Result<(), String> {
    let repo = AgentRepository::new(&db_state.db);
    repo.ensure_base_profiles(&memory_state.root_path)?;
    repo.update_default_model(&agent_id, default_model.as_deref())
}

#[tauri::command]
pub async fn agent_update_voice_settings(
    db_state: State<'_, DatabaseState>,
    memory_state: State<'_, MemoryState>,
    agent_id: String,
    voice_settings: AgentVoiceSettingsDto,
) -> Result<(), String> {
    let repo = AgentRepository::new(&db_state.db);
    repo.ensure_base_profiles(&memory_state.root_path)?;
    repo.update_voice_settings(
        &agent_id,
        voice_settings.enable_voice_output,
        voice_settings.piper_voice_preset.as_deref(),
        voice_settings.piper_model_path.as_deref(),
        voice_settings.enable_voice_input,
        voice_settings.whisper_model_path.as_deref(),
        voice_settings.whisper_language.as_deref(),
    )
}
