use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub env: HashMap<String, String>,

    #[serde(
        rename = "enabledPlugins",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub enabled_plugins: HashMap<String, bool>,

    // ponytail: Value not typed structs — real JSON is {source:{source,repo}},
    // DESIGN.md nested MarketplaceSourceInner one level too deep and would break round-trip
    #[serde(
        rename = "extraKnownMarketplaces",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub extra_known_marketplaces: HashMap<String, Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn round_trip_keeps_unknown_fields() {
        let raw = json!({
            "env": { "FOO": "bar", "ANTHROPIC_MODEL": "claude-sonnet-4-6" },
            "statusLine": { "type": "command", "command": "echo hi" },
            "theme": "light"
        });
        let s: Settings = serde_json::from_value(raw.clone()).unwrap();
        let out = serde_json::to_value(&s).unwrap();
        assert_eq!(out["statusLine"]["type"], "command");
        assert_eq!(out["env"]["FOO"], "bar");
        assert_eq!(out["theme"], "light");
        assert!(out.get("enabledPlugins").is_none());
    }

    #[test]
    fn marketplace_shape_matches_claude_code() {
        let raw = json!({
            "enabledPlugins": { "caveman@caveman": true },
            "extraKnownMarketplaces": {
                "caveman": { "source": { "source": "github", "repo": "JuliusBrussee/caveman" } }
            }
        });
        let s: Settings = serde_json::from_value(raw).unwrap();
        let out = serde_json::to_value(&s).unwrap();
        assert_eq!(
            out["extraKnownMarketplaces"]["caveman"]["source"]["repo"],
            "JuliusBrussee/caveman"
        );
        assert!(out["extraKnownMarketplaces"]["caveman"]["source"]["source"].is_string());
    }
}
