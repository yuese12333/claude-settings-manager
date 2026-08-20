use crate::commands::paths::app_data_file;
use crate::utils::atomic_write;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

fn validate(profiles: &[Profile]) -> Result<(), String> {
    let mut names = std::collections::HashSet::new();
    for p in profiles {
        if p.id.trim().is_empty() || p.name.trim().is_empty() {
            return Err("预设名称不能为空".into());
        }
        if !names.insert(p.name.trim().to_lowercase()) {
            return Err(format!("预设名称已存在：{}", p.name.trim()));
        }
    }
    Ok(())
}

#[tauri::command]
pub fn load_profiles(app: AppHandle) -> Result<Vec<Profile>, String> {
    let path = app_data_file(&app, "profiles.json")?;
    if !path.is_file() {
        return Ok(vec![]);
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("无法读取连接预设：{e}"))?;
    serde_json::from_str(raw.trim_start_matches('\u{feff}')).map_err(|e| format!("连接预设文件已损坏：{e}"))
}

#[tauri::command]
pub fn save_profiles(app: AppHandle, profiles: Vec<Profile>) -> Result<(), String> {
    validate(&profiles)?;
    let path = app_data_file(&app, "profiles.json")?;
    let json = serde_json::to_string_pretty(&profiles).map_err(|e| format!("序列化失败: {e}"))?;
    atomic_write(&path, &format!("{json}\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_duplicate_names() {
        let a = Profile {
            id: "1".into(),
            name: "官方".into(),
            base_url: "https://a.example".into(),
            api_key: "sk-a".into(),
        };
        let b = Profile {
            id: "2".into(),
            name: "官方".into(),
            base_url: "https://b.example".into(),
            api_key: "sk-b".into(),
        };
        assert!(validate(&[a, b]).is_err());
    }
}
