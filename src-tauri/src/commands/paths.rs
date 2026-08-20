use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

pub fn validate_settings_path(path: &Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("请使用绝对路径".into());
    }
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if name != "settings.json" && name != "settings.local.json" {
        return Err("仅支持 settings.json 或 settings.local.json".into());
    }
    Ok(())
}

pub fn app_data_file(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| format!("创建应用数据目录失败: {e}"))?;
    Ok(dir.join(name))
}

fn last_path_file(app: &AppHandle) -> Result<PathBuf, String> {
    app_data_file(app, "last_path.txt")
}

pub fn remember_path(app: &AppHandle, path: &str) -> Result<(), String> {
    fs::write(last_path_file(app)?, path).map_err(|e| format!("记录路径失败: {e}"))
}

fn read_last_path(app: &AppHandle) -> Option<PathBuf> {
    let file = last_path_file(app).ok()?;
    let s = fs::read_to_string(file).ok()?;
    let p = PathBuf::from(s.trim());
    if validate_settings_path(&p).is_ok() && p.is_file() {
        Some(p)
    } else {
        None
    }
}

fn windows_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(appdata) = std::env::var("APPDATA") {
        out.push(PathBuf::from(appdata).join("Claude").join("settings.json"));
    }
    if let Ok(home) = std::env::var("USERPROFILE") {
        out.push(PathBuf::from(home).join(".claude").join("settings.json"));
    }
    out
}

#[tauri::command]
pub fn detect_settings_path(app: AppHandle) -> Result<Option<String>, String> {
    if let Some(p) = read_last_path(&app) {
        return Ok(Some(p.to_string_lossy().into_owned()));
    }
    // ponytail: v1 Windows only — macOS/Linux candidates skipped
    for p in windows_candidates() {
        if p.is_file() {
            return Ok(Some(p.to_string_lossy().into_owned()));
        }
    }
    Ok(None)
}

#[tauri::command]
pub fn pick_settings_file(app: AppHandle) -> Result<Option<String>, String> {
    let picked = app
        .dialog()
        .file()
        .add_filter("Claude settings", &["json"])
        .set_title("选择 Claude Code 配置文件")
        .blocking_pick_file();

    let Some(file) = picked else {
        return Ok(None);
    };
    let path = file.into_path().map_err(|e| e.to_string())?;
    validate_settings_path(&path)?;
    let s = path.to_string_lossy().into_owned();
    remember_path(&app, &s)?;
    Ok(Some(s))
}

#[tauri::command]
pub fn sibling_settings_path(path: String) -> Option<String> {
    let p = Path::new(&path);
    let name = p.file_name()?.to_str()?;
    let other = match name {
        "settings.json" => "settings.local.json",
        "settings.local.json" => "settings.json",
        _ => return None,
    };
    let sib = p.with_file_name(other);
    sib.is_file().then(|| sib.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_relative_and_wrong_name() {
        assert!(validate_settings_path(Path::new("settings.json")).is_err());
        assert!(validate_settings_path(Path::new(r"C:\tmp\foo.json")).is_err());
        assert!(validate_settings_path(Path::new(r"C:\Users\ck\.claude\settings.json")).is_ok());
        assert!(
            validate_settings_path(Path::new(r"C:\Users\ck\.claude\settings.local.json")).is_ok()
        );
    }
}
