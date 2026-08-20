use crate::commands::paths::{remember_path, validate_settings_path};
use crate::models::Settings;
use crate::utils::atomic_write;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tauri::AppHandle;

#[derive(Debug, Serialize, Clone)]
pub struct ValidateOut {
    pub ok: bool,
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

fn type_cn(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "布尔值",
        Value::Number(_) => "数字",
        Value::String(_) => "字符串",
        Value::Array(_) => "数组",
        Value::Object(_) => "对象",
    }
}

fn locate(content: &str, needle: &str) -> (u32, u32) {
    let Some(idx) = content.find(needle) else {
        return (1, 1);
    };
    let before = &content[..idx];
    let line = before.bytes().filter(|&b| b == b'\n').count() as u32 + 1;
    let col = before.rsplit('\n').next().map(|s| s.chars().count()).unwrap_or(0) as u32 + 1;
    (line, col)
}

fn fail(content: &str, needle: &str, message: impl Into<String>) -> ValidateOut {
    let (line, column) = locate(content, needle);
    ValidateOut {
        ok: false,
        message: format!("第 {line} 行：{}", message.into()),
        line: Some(line),
        column: Some(column),
    }
}

/// http(s) Base URL：须带协议与主机名（不引入 url 依赖）
fn check_base_url(s: &str) -> Result<(), &'static str> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(());
    }
    if s.contains(char::is_whitespace) {
        return Err("`env.ANTHROPIC_BASE_URL` 不能包含空白字符");
    }
    let rest = if let Some(r) = s.strip_prefix("https://") {
        r
    } else if let Some(r) = s.strip_prefix("http://") {
        r
    } else {
        return Err("`env.ANTHROPIC_BASE_URL` 须以 http:// 或 https:// 开头");
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    if host.is_empty() {
        return Err("`env.ANTHROPIC_BASE_URL` 缺少主机名");
    }
    let hostname = host.split(':').next().unwrap_or("");
    if hostname.is_empty() {
        return Err("`env.ANTHROPIC_BASE_URL` 主机名无效");
    }
    if hostname == "localhost" || hostname.parse::<std::net::Ipv4Addr>().is_ok() {
        return Ok(());
    }
    if !hostname.contains('.') {
        return Err("`env.ANTHROPIC_BASE_URL` 主机名无效（须为域名、IP 或 localhost）");
    }
    Ok(())
}

fn check_timeout_ms(s: &str) -> Result<(), &'static str> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(());
    }
    let Ok(n) = s.parse::<u64>() else {
        return Err("`env.API_TIMEOUT_MS` 须为正整数字符串（毫秒），例如 \"600000\"");
    };
    if n == 0 {
        return Err("`env.API_TIMEOUT_MS` 须大于 0");
    }
    Ok(())
}

/// Claude Code settings.json 字段语义校验（非通用 JSON linter）
fn check_settings(content: &str, value: &Value) -> Option<ValidateOut> {
    let Some(root) = value.as_object() else {
        return Some(ValidateOut {
            ok: false,
            message: "第 1 行：settings.json 根节点须为对象 {}".into(),
            line: Some(1),
            column: Some(1),
        });
    };

    if let Some(env) = root.get("env") {
        let Some(map) = env.as_object() else {
            return Some(fail(
                content,
                "\"env\"",
                format!("`env` 须为对象（环境变量名 → 字符串），实际为 {}", type_cn(env)),
            ));
        };
        for (k, v) in map {
            let Some(s) = v.as_str() else {
                return Some(fail(
                    content,
                    &format!("\"{k}\""),
                    format!("`env.{k}` 须为字符串，实际为 {}（Claude Code 环境变量均为字符串）", type_cn(v)),
                ));
            };
            match k.as_str() {
                "ANTHROPIC_BASE_URL" => {
                    if let Err(msg) = check_base_url(s) {
                        return Some(fail(content, "\"ANTHROPIC_BASE_URL\"", msg));
                    }
                }
                "ANTHROPIC_AUTH_TOKEN" => {
                    if s.trim().is_empty() {
                        return Some(fail(
                            content,
                            "\"ANTHROPIC_AUTH_TOKEN\"",
                            "`env.ANTHROPIC_AUTH_TOKEN` 不能为空字符串；不用时请删除该键",
                        ));
                    }
                }
                "ANTHROPIC_MODEL" => {
                    if s.trim().is_empty() {
                        return Some(fail(
                            content,
                            "\"ANTHROPIC_MODEL\"",
                            "`env.ANTHROPIC_MODEL` 不能为空；请填模型 ID，例如 claude-sonnet-4-6",
                        ));
                    }
                }
                "API_TIMEOUT_MS" => {
                    if let Err(msg) = check_timeout_ms(s) {
                        return Some(fail(content, "\"API_TIMEOUT_MS\"", msg));
                    }
                }
                "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC" => {
                    if s != "1" {
                        return Some(fail(
                            content,
                            "\"CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC\"",
                            "`env.CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 启用时应为 \"1\"；关闭则删除该键",
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(theme) = root.get("theme") {
        match theme {
            Value::Null => {}
            Value::String(t) => {
                if t != "light" && t != "dark" {
                    return Some(fail(
                        content,
                        "\"theme\"",
                        format!("`theme` 仅支持 \"light\" 或 \"dark\"，当前为 \"{t}\""),
                    ));
                }
            }
            other => {
                return Some(fail(
                    content,
                    "\"theme\"",
                    format!("`theme` 须为 \"light\" / \"dark\" 或 null，实际为 {}", type_cn(other)),
                ));
            }
        }
    }

    if let Some(plugins) = root.get("enabledPlugins") {
        let Some(map) = plugins.as_object() else {
            return Some(fail(
                content,
                "\"enabledPlugins\"",
                format!(
                    "`enabledPlugins` 须为对象（插件 ID → true/false），实际为 {}",
                    type_cn(plugins)
                ),
            ));
        };
        for (id, v) in map {
            if id.trim().is_empty() {
                return Some(fail(
                    content,
                    "\"enabledPlugins\"",
                    "`enabledPlugins` 的键不能为空；格式一般为 pluginName@marketplace",
                ));
            }
            if !v.is_boolean() {
                return Some(fail(
                    content,
                    &format!("\"{id}\""),
                    format!("`enabledPlugins.{id}` 须为布尔值 true/false，实际为 {}", type_cn(v)),
                ));
            }
            if !id.contains('@') {
                return Some(fail(
                    content,
                    &format!("\"{id}\""),
                    format!(
                        "`enabledPlugins` 键 \"{id}\" 格式不符：期望 pluginName@marketplace（例如 caveman@caveman）"
                    ),
                ));
            }
        }
    }

    if let Some(markets) = root.get("extraKnownMarketplaces") {
        let Some(map) = markets.as_object() else {
            return Some(fail(
                content,
                "\"extraKnownMarketplaces\"",
                format!(
                    "`extraKnownMarketplaces` 须为对象（marketplace 名 → 源配置），实际为 {}",
                    type_cn(markets)
                ),
            ));
        };
        for (name, entry) in map {
            let Some(obj) = entry.as_object() else {
                return Some(fail(
                    content,
                    &format!("\"{name}\""),
                    format!(
                        "`extraKnownMarketplaces.{name}` 须为对象，实际为 {}",
                        type_cn(entry)
                    ),
                ));
            };
            let Some(source) = obj.get("source") else {
                return Some(fail(
                    content,
                    &format!("\"{name}\""),
                    format!("`extraKnownMarketplaces.{name}` 缺少 `source` 字段"),
                ));
            };
            let Some(src) = source.as_object() else {
                return Some(fail(
                    content,
                    "\"source\"",
                    format!(
                        "`extraKnownMarketplaces.{name}.source` 须为对象，实际为 {}",
                        type_cn(source)
                    ),
                ));
            };
            let Some(kind) = src.get("source").and_then(|x| x.as_str()) else {
                return Some(fail(
                    content,
                    "\"source\"",
                    format!(
                        "`extraKnownMarketplaces.{name}.source.source` 须为字符串（如 \"github\"）"
                    ),
                ));
            };
            if kind == "github" {
                let Some(repo) = src.get("repo").and_then(|x| x.as_str()) else {
                    return Some(fail(
                        content,
                        "\"repo\"",
                        format!(
                            "`extraKnownMarketplaces.{name}` 为 github 源时须提供 `source.repo`（owner/name）"
                        ),
                    ));
                };
                if !repo.contains('/') || repo.starts_with('/') || repo.ends_with('/') {
                    return Some(fail(
                        content,
                        "\"repo\"",
                        format!(
                            "`extraKnownMarketplaces.{name}.source.repo` 须为 owner/name，当前为 \"{repo}\""
                        ),
                    ));
                }
            }
        }
    }

    None
}

fn parse_settings(content: &str) -> Result<Settings, ValidateOut> {
    let raw = content.trim_start_matches('\u{feff}');
    let value: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            return Err(ValidateOut {
                ok: false,
                message: format!(
                    "第 {} 行第 {} 列：settings.json 无法解析（{}）",
                    e.line(),
                    e.column(),
                    e
                ),
                line: Some(e.line() as u32),
                column: Some(e.column() as u32),
            });
        }
    };
    if !value.is_object() {
        return Err(ValidateOut {
            ok: false,
            message: "第 1 行：settings.json 根节点须为对象 {}".into(),
            line: Some(1),
            column: Some(1),
        });
    }
    serde_json::from_value(value).map_err(|e| ValidateOut {
        ok: false,
        message: format!("settings.json 结构不符：{e}"),
        line: None,
        column: None,
    })
}

/// 源文件页 / 原始保存：Claude Code 字段语义校验
fn validate_content(content: &str) -> Result<(Settings, ValidateOut), ValidateOut> {
    let raw = content.trim_start_matches('\u{feff}');
    let value: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            return Err(ValidateOut {
                ok: false,
                message: format!(
                    "第 {} 行第 {} 列：settings.json 无法解析（{}）",
                    e.line(),
                    e.column(),
                    e
                ),
                line: Some(e.line() as u32),
                column: Some(e.column() as u32),
            });
        }
    };

    if let Some(bad) = check_settings(raw, &value) {
        return Err(bad);
    }

    match serde_json::from_value::<Settings>(value) {
        Ok(settings) => Ok((
            settings,
            ValidateOut {
                ok: true,
                message: "settings.json 字段校验通过".into(),
                line: None,
                column: None,
            },
        )),
        Err(e) => Err(ValidateOut {
            ok: false,
            message: format!("settings.json 结构不符：{e}"),
            line: None,
            column: None,
        }),
    }
}

#[tauri::command]
pub fn load_settings(app: AppHandle, path: String) -> Result<Settings, String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let raw = fs::read_to_string(p).map_err(|e| format!("读取失败: {e}"))?;
    // 加载只要求能解析为 Settings，便于打开后在源文件页修正字段问题
    let settings = parse_settings(&raw).map_err(|v| v.message)?;
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
pub fn validate_settings_json(content: String) -> ValidateOut {
    match validate_content(&content) {
        Ok((_, ok)) => ok,
        Err(bad) => ValidateOut {
            ok: false,
            message: format!("保存后可能无法正常使用 — {}", bad.message),
            line: bad.line,
            column: bad.column,
        },
    }
}

#[tauri::command]
pub fn save_settings(path: String, settings: Settings) -> Result<(), String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let json = serde_json::to_string_pretty(&settings).map_err(|e| format!("序列化失败: {e}"))?;
    atomic_write(p, &format!("{json}\n"))
}

/// 始终写入原文；能解析则返回 Settings，否则 settings=None（不拦截保存）
#[derive(Debug, Serialize)]
pub struct SaveRawOut {
    pub settings: Option<Settings>,
}

#[tauri::command]
pub fn save_settings_raw(path: String, content: String) -> Result<SaveRawOut, String> {
    let p = Path::new(&path);
    validate_settings_path(p)?;
    let body = content.trim_start_matches('\u{feff}');
    let body = if body.ends_with('\n') {
        body.to_string()
    } else {
        format!("{body}\n")
    };
    atomic_write(p, &body)?;
    Ok(SaveRawOut {
        settings: parse_settings(&body).ok(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_base_url() {
        let bad = validate_settings_json(
            r#"{"env":{"ANTHROPIC_BASE_URL":"not-a-url"}}"#.into(),
        );
        assert!(!bad.ok);
        assert!(bad.message.contains("保存后可能无法正常使用"));
        assert!(bad.message.contains("ANTHROPIC_BASE_URL"));
        assert!(bad.message.contains("http"));
    }

    #[test]
    fn rejects_bad_timeout() {
        let bad = validate_settings_json(r#"{"env":{"API_TIMEOUT_MS":"abc"}}"#.into());
        assert!(!bad.ok);
        assert!(bad.message.contains("API_TIMEOUT_MS"));
    }

    #[test]
    fn rejects_bad_theme() {
        let bad = validate_settings_json(r#"{"theme":"blue"}"#.into());
        assert!(!bad.ok);
        assert!(bad.message.contains("theme"));
    }

    #[test]
    fn rejects_plugin_id_without_at() {
        let bad = validate_settings_json(r#"{"enabledPlugins":{"caveman":true}}"#.into());
        assert!(!bad.ok);
        assert!(bad.message.contains("@"));
    }

    #[test]
    fn accepts_valid_settings() {
        let ok = validate_settings_json(
            r#"{
              "env": {
                "ANTHROPIC_BASE_URL": "https://api.anthropic.com",
                "ANTHROPIC_AUTH_TOKEN": "sk-test",
                "ANTHROPIC_MODEL": "claude-sonnet-4-6",
                "API_TIMEOUT_MS": "600000"
              },
              "enabledPlugins": { "caveman@caveman": true },
              "extraKnownMarketplaces": {
                "caveman": { "source": { "source": "github", "repo": "JuliusBrussee/caveman" } }
              },
              "theme": "light"
            }"#
            .into(),
        );
        assert!(ok.ok, "{}", ok.message);
    }

    #[test]
    fn allows_unknown_env_as_string() {
        let ok = validate_settings_json(r#"{"env":{"MY_CUSTOM":"x"}}"#.into());
        assert!(ok.ok);
    }
}
