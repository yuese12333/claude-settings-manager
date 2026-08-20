use crate::commands::paths::{remember_path, validate_settings_path};
use crate::models::Settings;
use crate::utils::atomic_write;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tauri::AppHandle;

fn validate_content(content: &str) -> Result<Settings, String> {
    let raw = content.trim_start_matches('\u{feff}');
    let value: Value =
        serde_json::from_str(raw).map_err(|e| format!("JSON 语法无效：{e}"))?;
    if !value.is_object() {
        return Err("根节点须为 JSON 对象".into());
    }
    serde_json::from_value(value).map_err(|e| format!("不符合 Claude Code settings 结构：{e}"))
}

#[tauri::command]
pub fn load_settings(app: AppHandle, path: String) -> Result<Settings, String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let raw = fs::read_to_string(p).map_err(|e| format!("读取失败: {e}"))?;
    let settings = validate_content(&raw)?;
    remember_path(&app, &path)?;
    Ok(settings)
}

#[tauri::command]
pub fn load_settings_raw(path: String) -> Result<String, String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let raw = fs::read_to_string(p).map_err(|e| format!("读取失败: {e}"))?;
    Ok(raw.trim_start_matches('\u{feff}').to_string())
}

#[tauri::command]
pub fn validate_settings_json(content: String) -> Result<(), String> {
    validate_content(&content).map(|_| ())
}

#[tauri::command]
pub fn save_settings(path: String, settings: Settings) -> Result<(), String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let json = serde_json::to_string_pretty(&settings).map_err(|e| format!("序列化失败: {e}"))?;
    atomic_write(p, &format!("{json}\n"))
}

#[tauri::command]
pub fn save_settings_raw(path: String, content: String) -> Result<Settings, String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let settings = validate_content(&content)?;
    let body = content.trim_start_matches('\u{feff}');
    let body = if body.ends_with('\n') {
        body.to_string()
    } else {
        format!("{body}\n")
    };
    atomic_write(p, &body)?;
    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_object_and_bad_env() {
        assert!(validate_content("[]").is_err());
        assert!(validate_content(r#"{"env":"nope"}"#).is_err());
        assert!(validate_content(r#"{"env":{"A":"1"},"theme":"light"}"#).is_ok());
    }
}
