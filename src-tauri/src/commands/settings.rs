use crate::commands::paths::{remember_path, validate_settings_path};
use crate::models::Settings;
use crate::utils::atomic_write;
use std::fs;
use std::path::Path;
use tauri::AppHandle;

#[tauri::command]
pub fn load_settings(app: AppHandle, path: String) -> Result<Settings, String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let raw = fs::read_to_string(p).map_err(|e| format!("读取失败: {e}"))?;
    let raw = raw.trim_start_matches('\u{feff}');
    let settings: Settings =
        serde_json::from_str(raw).map_err(|e| format!("JSON 解析失败: {e}"))?;
    remember_path(&app, &path)?;
    Ok(settings)
}

#[tauri::command]
pub fn save_settings(path: String, settings: Settings) -> Result<(), String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let json = serde_json::to_string_pretty(&settings).map_err(|e| format!("序列化失败: {e}"))?;
    atomic_write(p, &format!("{json}\n"))
}
