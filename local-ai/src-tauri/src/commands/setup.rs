use std::path::Path;
use std::process::{Command, Stdio};

use tauri::State;

use crate::DatabaseState;

#[tauri::command]
pub async fn setup_open_external(target: String) -> Result<(), String> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return Err("No external target was provided.".to_string());
    }

    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("cmd");
        command.arg("/C").arg("start").arg("").arg(trimmed);
        command
    };

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("open");
        command.arg(trimmed);
        command
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = Command::new("xdg-open");
        command.arg(trimmed);
        command
    };

    command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("Failed to open external target: {}", error))?;

    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn setup_start_ollama() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("cmd");
        command.arg("/C").arg("start").arg("").arg("ollama").arg("serve");
        command
    };

    #[cfg(not(target_os = "windows"))]
    let mut command = {
        let mut command = Command::new("ollama");
        command.arg("serve");
        command
    };

    command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| {
            format!(
                "Failed to start Ollama automatically: {}. If Ollama is not installed yet, install it first.",
                error
            )
        })?;

    Ok(())
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn setup_start_ollama(state: State<'_, DatabaseState>) -> Result<(), String> {
    let configured_executable = read_string_setting(&state, "directEngineExecutablePath")?;
    let executable = resolve_llama_server_path(configured_executable.as_deref())?;

    let model_path = read_string_setting(&state, "directEngineModelPath")?
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            "No GGUF model path is configured yet. Add it in Settings under llama-server Executable / GGUF Model Path, then try Start Engine again.".to_string()
        })?;

    if !Path::new(&model_path).exists() {
        return Err(format!(
            "The configured GGUF model was not found at {}. Update the GGUF Model Path in Settings and try again.",
            model_path
        ));
    }

    let mut command = Command::new(&executable);
    command.arg("-m").arg(&model_path);

    if let Some(mmproj_path) = infer_mmproj_path(&model_path) {
        command.arg("--mmproj").arg(mmproj_path);
    }

    if let Some(alias) = infer_model_alias(&model_path) {
        command.arg("--alias").arg(alias);
    }

    command
        .arg("--host")
        .arg("127.0.0.1")
        .arg("--port")
        .arg("8080")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("Failed to start llama.cpp server: {}", error))?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn read_string_setting(state: &State<'_, DatabaseState>, key: &str) -> Result<Option<String>, String> {
    let value = state.db.get_setting(key)?;

    Ok(value.and_then(|raw| {
        serde_json::from_str::<String>(&raw)
            .ok()
            .or_else(|| {
                let trimmed = raw.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
    }))
}

#[cfg(target_os = "macos")]
fn resolve_llama_server_path(configured: Option<&str>) -> Result<String, String> {
    let candidates = configured
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .chain(
            [
                "/opt/homebrew/bin/llama-server".to_string(),
                "/usr/local/bin/llama-server".to_string(),
            ]
            .into_iter(),
        );

    for candidate in candidates {
        if Path::new(&candidate).exists() {
            return Ok(candidate);
        }
    }

    Err(
        "Could not find llama-server. Install llama.cpp first, or set the full llama-server path in Settings."
            .to_string(),
    )
}

#[cfg(target_os = "macos")]
fn infer_mmproj_path(model_path: &str) -> Option<String> {
    let model = Path::new(model_path);
    let parent = model.parent()?;
    let mut entries = std::fs::read_dir(parent).ok()?;

    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();
        let file_name = path.file_name()?.to_str()?.to_ascii_lowercase();
        if file_name.ends_with(".gguf") && file_name.starts_with("mmproj-") {
            return Some(path.to_string_lossy().to_string());
        }
    }

    None
}

#[cfg(target_os = "macos")]
fn infer_model_alias(model_path: &str) -> Option<&'static str> {
    let lower = Path::new(model_path)
        .file_name()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())?;

    if lower.contains("gemma-4-e4b") {
        Some("google/gemma-4-e4b")
    } else if lower.contains("gemma-4-e2b") {
        Some("google/gemma-4-e2b")
    } else {
        None
    }
}
