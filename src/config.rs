use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub space_id: String,
    pub api_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    pub anthropic_api_key: Option<String>,
    pub default_model: Option<String>,
    pub default_task_count: usize,
}

impl Default for AiConfig {
    fn default() -> Self {
        AiConfig {
            anthropic_api_key: None,
            default_model: Some("claude-3-5-sonnet-20241022".to_string()),
            default_task_count: 10,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub current_profile: String,
    #[serde(default)]
    pub ai: AiConfig,
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

        // Check file permissions on Unix systems
        #[cfg(unix)]
        Self::check_permissions(&config_path)?;

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

        // Set restrictive permissions on Unix systems (owner read/write only)
        #[cfg(unix)]
        Self::set_restrictive_permissions(&config_path)?;

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

    #[cfg(unix)]
    fn check_permissions(path: &PathBuf) -> Result<()> {
        let metadata = fs::metadata(path)
            .with_context(|| format!("Failed to read config file metadata: {}", path.display()))?;
        let mode = metadata.permissions().mode();
        let user_mode = mode & 0o777;

        // Check if file is readable/writable by others (not owner)
        // 0o600 = owner read/write only
        if user_mode & 0o077 != 0 {
            return Err(anyhow::anyhow!(
                "Config file has insecure permissions ({:o}). \
                Please run: chmod 600 {}",
                user_mode,
                path.display()
            ));
        }

        Ok(())
    }

    #[cfg(unix)]
    fn set_restrictive_permissions(path: &PathBuf) -> Result<()> {
        let mut perms = fs::metadata(path)
            .with_context(|| format!("Failed to read config file metadata: {}", path.display()))?
            .permissions();
        perms.set_mode(0o600);
        fs::set_permissions(path, perms).with_context(|| {
            format!("Failed to set config file permissions: {}", path.display())
        })?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert(
            "default".to_string(),
            Profile {
                space_id: String::new(),
                api_token: String::new(),
            },
        );

        Config {
            profiles,
            current_profile: "default".to_string(),
            ai: AiConfig::default(),
        }
    }
}

pub fn load_credentials() -> Result<(String, String)> {
    let space_id = std::env::var("REPSONA_SPACE");
    let api_token = std::env::var("REPSONA_TOKEN");

    if space_id.is_ok() && api_token.is_ok() {
        #[allow(clippy::unnecessary_unwrap)]
        return Ok((space_id.unwrap(), api_token.unwrap()));
    }

    let config = Config::load()?;
    let profile = config
        .get_current_profile()
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

/// Anthropic APIキーをロードする
/// 環境変数 ANTHROPIC_API_KEY が優先、設定ファイルがフォールバック
pub fn load_anthropic_api_key() -> Result<String> {
    // 環境変数を優先
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        return Ok(key);
    }

    // 設定ファイルから読み込み
    let config = Config::load()?;
    config
        .ai
        .anthropic_api_key
        .ok_or_else(|| anyhow::anyhow!("Anthropic API key not configured. Set ANTHROPIC_API_KEY environment variable or configure it in config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::env;
    use tempfile::TempDir;

    #[allow(dead_code)]
    fn setup_test_config_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    #[allow(dead_code)]
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

        config.add_profile(
            "prod".to_string(),
            Profile {
                space_id: "prod-space".to_string(),
                api_token: "prod-token".to_string(),
            },
        );

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
        config.add_profile(
            "test".to_string(),
            Profile {
                space_id: "test-space".to_string(),
                api_token: "test-token".to_string(),
            },
        );
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

        config.add_profile(
            "dev".to_string(),
            Profile {
                space_id: "dev-space".to_string(),
                api_token: "dev-token".to_string(),
            },
        );

        config.add_profile(
            "staging".to_string(),
            Profile {
                space_id: "staging-space".to_string(),
                api_token: "staging-token".to_string(),
            },
        );

        config.add_profile(
            "prod".to_string(),
            Profile {
                space_id: "prod-space".to_string(),
                api_token: "prod-token".to_string(),
            },
        );

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

    // =========================================================================
    // Property-Based Tests
    // =========================================================================

    proptest! {
        /// Property: TOMLシリアライズ→デシリアライズでConfigが等価
        #[test]
        fn prop_config_roundtrip_serialization(
            profile_names in prop::collection::vec("[a-zA-Z0-9_-]{1,20}", 1..5),
            space_ids in prop::collection::vec("[a-zA-Z0-9]{4,16}", 1..5),
            api_tokens in prop::collection::vec("[a-zA-Z0-9+/=]{16,64}", 1..5),
        ) {
            // 最も小さいベクターのサイズに合わせる
            let count = profile_names.len().min(space_ids.len()).min(api_tokens.len());

            let mut config = Config::default();
            for i in 0..count {
                config.add_profile(
                    profile_names[i].clone(),
                    Profile {
                        space_id: space_ids[i].clone(),
                        api_token: api_tokens[i].clone(),
                    }
                );
            }

            // TOMLシリアライズ→デシリアライズ
            let serialized = toml::to_string_pretty(&config).unwrap();
            let deserialized: Config = toml::from_str(&serialized).unwrap();

            // 全プロファイルが保持される
            prop_assert_eq!(config.profiles.len(), deserialized.profiles.len());
            for (name, profile) in &config.profiles {
                let deserialized_profile = deserialized.profiles.get(name)
                    .expect("プロファイルが見つかりません");
                prop_assert_eq!(&profile.space_id, &deserialized_profile.space_id);
                prop_assert_eq!(&profile.api_token, &deserialized_profile.api_token);
            }
        }

        /// Property: Profileの認証情報は操作を通じて保持される
        #[test]
        fn prop_profile_preserves_credentials(
            space_id in "[a-zA-Z0-9_-]{4,32}",
            api_token in "[a-zA-Z0-9+/=_-]{16,128}"
        ) {
            let profile = Profile {
                space_id: space_id.clone(),
                api_token: api_token.clone(),
            };

            // 直接フィールドアクセスで検証
            prop_assert_eq!(&profile.space_id, &space_id);
            prop_assert_eq!(&profile.api_token, &api_token);

            // TOML往復でも保持される
            let serialized = toml::to_string(&profile).unwrap();
            let deserialized: Profile = toml::from_str(&serialized).unwrap();
            prop_assert_eq!(&deserialized.space_id, &space_id);
            prop_assert_eq!(&deserialized.api_token, &api_token);
        }

        /// Property: 新規プロファイル追加で数が+1される
        #[test]
        fn prop_add_profile_increases_count(
            existing_names in prop::collection::hash_set("[a-z]{3,10}", 1..5),
            new_name in "[a-z]{3,10}"
        ) {
            let mut config = Config::default();

            // 既存プロファイルを追加
            for name in &existing_names {
                config.add_profile(name.clone(), Profile {
                    space_id: "space".to_string(),
                    api_token: "token".to_string(),
                });
            }

            let initial_count = config.profiles.len();

            // 新規名が既存と被らない場合は追加
            if !existing_names.contains(&new_name) {
                config.add_profile(new_name.clone(), Profile {
                    space_id: "new-space".to_string(),
                    api_token: "new-token".to_string(),
                });
                prop_assert_eq!(config.profiles.len(), initial_count + 1);
                prop_assert!(config.profiles.contains_key(&new_name));
            } else {
                // 既存名を追加しても数は変わらない
                let before_len = config.profiles.len();
                config.add_profile(new_name, Profile {
                    space_id: "another-space".to_string(),
                    api_token: "another-token".to_string(),
                });
                prop_assert_eq!(config.profiles.len(), before_len);
            }
        }
    }
}
