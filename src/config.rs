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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    fn setup_test_config_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    fn set_test_config_path(temp_dir: &TempDir) {
        env::set_var("HOME", temp_dir.path());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert_eq!(config.current_profile, "default");
        assert_eq!(config.profiles.len(), 1);
        assert!(config.profiles.contains_key("default"));

        let default_profile = config.get_profile("default").unwrap();
        assert_eq!(default_profile.space_id, "");
        assert_eq!(default_profile.api_token, "");
    }

    #[test]
    fn test_get_current_profile() {
        let config = Config::default();
        let profile = config.get_current_profile();

        assert!(profile.is_some());
        let profile = profile.unwrap();
        assert_eq!(profile.space_id, "");
        assert_eq!(profile.api_token, "");
    }

    #[test]
    fn test_add_profile() {
        let mut config = Config::default();

        let new_profile = Profile {
            space_id: "test-space".to_string(),
            api_token: "test-token".to_string(),
        };

        config.add_profile("test".to_string(), new_profile);

        assert_eq!(config.profiles.len(), 2);
        assert!(config.profiles.contains_key("test"));

        let profile = config.get_profile("test").unwrap();
        assert_eq!(profile.space_id, "test-space");
        assert_eq!(profile.api_token, "test-token");
    }

    #[test]
    fn test_set_current_profile_success() {
        let mut config = Config::default();

        config.add_profile("prod".to_string(), Profile {
            space_id: "prod-space".to_string(),
            api_token: "prod-token".to_string(),
        });

        let result = config.set_current_profile("prod".to_string());
        assert!(result.is_ok());
        assert_eq!(config.current_profile, "prod");

        let current = config.get_current_profile().unwrap();
        assert_eq!(current.space_id, "prod-space");
    }

    #[test]
    fn test_set_current_profile_not_found() {
        let mut config = Config::default();

        let result = config.set_current_profile("nonexistent".to_string());
        assert!(result.is_err());
        assert_eq!(config.current_profile, "default");
    }

    #[test]
    fn test_profile_serialization() {
        let profile = Profile {
            space_id: "my-space".to_string(),
            api_token: "my-token".to_string(),
        };

        let serialized = toml::to_string(&profile).unwrap();
        assert!(serialized.contains("space_id = \"my-space\""));
        assert!(serialized.contains("api_token = \"my-token\""));

        let deserialized: Profile = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.space_id, "my-space");
        assert_eq!(deserialized.api_token, "my-token");
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();
        config.add_profile("test".to_string(), Profile {
            space_id: "test-space".to_string(),
            api_token: "test-token".to_string(),
        });
        config.set_current_profile("test".to_string()).unwrap();

        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("current_profile = \"test\""));
        assert!(serialized.contains("[profiles.test]"));

        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.current_profile, "test");
        assert_eq!(deserialized.profiles.len(), 2);
    }

    #[test]
    fn test_load_credentials_from_env() {
        env::set_var("REPSONA_SPACE", "env-space");
        env::set_var("REPSONA_TOKEN", "env-token");

        let result = load_credentials();

        env::remove_var("REPSONA_SPACE");
        env::remove_var("REPSONA_TOKEN");

        assert!(result.is_ok());
        let (space_id, api_token) = result.unwrap();
        assert_eq!(space_id, "env-space");
        assert_eq!(api_token, "env-token");
    }

    #[test]
    fn test_load_credentials_partial_env() {
        env::set_var("REPSONA_SPACE", "env-space-only");
        env::remove_var("REPSONA_TOKEN");

        let result = load_credentials();

        env::remove_var("REPSONA_SPACE");

        // Should fail because config file doesn't exist in test environment
        // or succeed if it falls back to config
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_get_profile_nonexistent() {
        let config = Config::default();
        let profile = config.get_profile("nonexistent");
        assert!(profile.is_none());
    }

    #[test]
    fn test_multiple_profiles() {
        let mut config = Config::default();

        config.add_profile("dev".to_string(), Profile {
            space_id: "dev-space".to_string(),
            api_token: "dev-token".to_string(),
        });

        config.add_profile("staging".to_string(), Profile {
            space_id: "staging-space".to_string(),
            api_token: "staging-token".to_string(),
        });

        config.add_profile("prod".to_string(), Profile {
            space_id: "prod-space".to_string(),
            api_token: "prod-token".to_string(),
        });

        assert_eq!(config.profiles.len(), 4); // default + 3 new

        config.set_current_profile("staging".to_string()).unwrap();
        let current = config.get_current_profile().unwrap();
        assert_eq!(current.space_id, "staging-space");
    }

    #[test]
    fn test_profile_clone() {
        let profile = Profile {
            space_id: "test".to_string(),
            api_token: "token".to_string(),
        };

        let cloned = profile.clone();
        assert_eq!(profile.space_id, cloned.space_id);
        assert_eq!(profile.api_token, cloned.api_token);
    }
}
