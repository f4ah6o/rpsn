use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub space_id: String,
    pub api_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub current_profile: String,
}

impl Config {
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("rpsn").join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn add_profile(&mut self, name: String, profile: Profile) {
        self.profiles.insert(name, profile);
    }

    pub fn set_current_profile(&mut self, name: String) -> Result<()> {
        if !self.profiles.contains_key(&name) {
            return Err(anyhow::anyhow!("Profile '{}' not found", name));
        }
        self.current_profile = name;
        Ok(())
    }

    pub fn get_current_profile(&self) -> Option<&Profile> {
        self.get_profile(&self.current_profile)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("default".to_string(), Profile {
            space_id: String::new(),
            api_token: String::new(),
        });

        Config {
            profiles,
            current_profile: "default".to_string(),
        }
    }
}

pub fn load_credentials() -> Result<(String, String)> {
    let space_id = std::env::var("REPSONA_SPACE");
    let api_token = std::env::var("REPSONA_TOKEN");

    if space_id.is_ok() && api_token.is_ok() {
        return Ok((space_id.unwrap(), api_token.unwrap()));
    }

    let config = Config::load()?;
    let profile = config.get_current_profile()
        .ok_or_else(|| anyhow::anyhow!("No current profile configured"))?;

    let space_id = if space_id.is_ok() {
        space_id?
    } else {
        profile.space_id.clone()
    };

    let api_token = if api_token.is_ok() {
        api_token?
    } else {
        profile.api_token.clone()
    };

    Ok((space_id, api_token))
}
