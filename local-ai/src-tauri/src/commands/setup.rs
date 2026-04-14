use std::process::{Command, Stdio};

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
pub async fn setup_start_ollama() -> Result<(), String> {
    let mut open_app = Command::new("open");
    open_app.arg("-a").arg("LM Studio");

    if open_app
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
    {
        return Ok(());
    }

    Err("Failed to open LM Studio automatically on macOS. Open /Applications/LM Studio.app, start the local server on port 1234, load a Gemma 4 model, then refresh setup.".to_string())
}
