use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Proxy configsuration loaded from configs/desktop-apps.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxyconfigs {
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub rules_directory: Option<PathBuf>,
    #[serde(default)]
    pub default_upstream: Option<String>,
    #[serde(default)]
    pub max_body_bytes: Option<u64>,
}

/// Per-app proxy rules (allow/block)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRule {
    pub id: String,
    #[serde(default)]
    pub allowlist: Vec<String>,
    #[serde(default)]
    pub blocklist: Vec<String>,
    #[serde(default)]
    pub rate_limit_rps: Option<u32>,
    #[serde(default)]
    pub notes: Option<String>,
}

/// Load all rule files (YAML/JSON) from a directory.
pub fn load_rules(dir: &Path) -> anyhow::Result<Vec<ProxyRule>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut rules = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let path = entry.path();
        let contents = fs::read_to_string(&path)?;
        let parsed = if path.extension().map(|e| e == "json").unwrap_or(false) {
            serde_json::from_str::<ProxyRule>(&contents)?
        } else {
            serde_yaml::from_str::<ProxyRule>(&contents)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to parse rule"))?
        };
        rules.push(parsed);
    }

    Ok(rules)
}

/// Build a default proxy configsuration when configs is missing.
pub fn default_proxy_configs() -> Proxyconfigs {
    Proxyconfigs {
        host: "127.0.0.1".into(),
        port: 8085,
        rules_directory: None,
        default_upstream: None,
        max_body_bytes: Some(10 * 1024 * 1024),
    }
}
