use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub glm_api_key: Option<String>,
    pub glm_model: Option<String>,
    pub glm_base_url: Option<String>,
    pub brave_api_key: Option<String>,
    pub groq_api_key: Option<String>,
    pub groq_model: Option<String>,
}

pub fn config_path() -> PathBuf {
    std::env::var("APP_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/data/config.json"))
}

pub fn load_config() -> AppConfig {
    let p = config_path();
    match fs::read_to_string(&p) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
        Err(_) => AppConfig::default(),
    }
}

pub fn save_config(cfg: &AppConfig) -> Result<()> {
    let p = config_path();
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(p, serde_json::to_string_pretty(cfg)?)?;
    Ok(())
}

pub fn get_setting(key: &str) -> Option<String> {
    let cfg = load_config();
    match key {
        "GLM_API_KEY" => cfg.glm_api_key,
        "GLM_MODEL" => cfg.glm_model,
        "GLM_BASE_URL" => cfg.glm_base_url,
        "BRAVE_API_KEY" => cfg.brave_api_key,
        "GROQ_API_KEY" => cfg.groq_api_key,
        "GROQ_MODEL" => cfg.groq_model,
        _ => None,
    }
}
