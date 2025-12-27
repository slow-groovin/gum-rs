//! # Configuration Management Module
//!
//! Responsible for application configuration management, including storage,
//! loading, and operations on user configurations. Uses parallel loading strategy
//! to fetch all needed configuration information at once during initialization.

use crate::utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::thread;
/// User configuration struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub name: String,
    pub email: String,
}

/// Main configuration struct
#[derive(Debug)]
pub struct Config {
    /// User defined configuration groups
    pub groups: HashMap<String, UserConfig>,
    /// Global git user configuration (cached)
    pub global_user: Option<UserConfig>,
    /// Project level git user configuration (cached)
    pub project_user: Option<UserConfig>,
}

/// Configuration file struct (only used for serialization/deserialization)
#[derive(Serialize, Deserialize)]
struct ConfigFile {
    groups: HashMap<String, UserConfig>,
}

impl Config {
    /// Create empty configuration instance
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            global_user: None,
            project_user: None,
        }
    }

    /// Load all configurations in parallel
    ///
    /// Executes three operations simultaneously:
    /// 1. Load user configuration groups from file
    /// 2. Get global git configuration
    /// 3. Get project git configuration
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        log::debug!("Starting parallel config loading");

        // Start three parallel tasks
        let file_handle = thread::spawn(|| load_config_file());
        let global_handle = thread::spawn(|| get_git_user_batch(true));
        let project_handle = thread::spawn(|| get_git_user_batch(false));

        // Wait for all tasks to complete
        let groups = file_handle
            .join()
            .map_err(|_| "Config file loading thread panicked")?
            .unwrap_or_else(|e| {
                log::warn!("Failed to load config file: {}", e);
                HashMap::new()
            });

        let global_user = global_handle
            .join()
            .map_err(|_| "Global git config loading thread panicked")?
            .ok();

        let project_user = project_handle
            .join()
            .map_err(|_| "Project git config loading thread panicked")?
            .ok();

        log::debug!(
            "Config loading complete: {} groups, global user: {}, project user: {}",
            groups.len(),
            global_user.is_some(),
            project_user.is_some()
        );

        Ok(Config {
            groups,
            global_user,
            project_user,
        })
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Saving configuration to file");
        let config_path = utils::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_file = ConfigFile {
            groups: self.groups.clone(),
        };

        let content = serde_json::to_string_pretty(&config_file)?;
        fs::write(config_path, content)?;
        log::debug!("Configuration saved successfully");
        Ok(())
    }

    /// Get currently used git user configuration
    ///
    /// Returns project configuration first, if not exists returns global configuration
    pub fn get_using_git_user(&self) -> Result<&UserConfig, Box<dyn std::error::Error>> {
        self.project_user
            .as_ref()
            .or(self.global_user.as_ref())
            .ok_or_else(|| "No git user configuration found".into())
    }

    /// Get all configuration information (including global configuration)
    pub fn get_all_config_info(&self) -> HashMap<String, UserConfig> {
        let mut all_info = self.groups.clone();
        if let Some(ref global_user) = self.global_user {
            all_info.insert("global".to_string(), global_user.clone());
        }
        all_info
    }

    /// Refresh global git configuration
    pub fn refresh_global_user(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.global_user = get_git_user_batch(true).ok();
        Ok(())
    }

    /// Refresh project git configuration
    pub fn refresh_project_user(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.project_user = get_git_user_batch(false).ok();
        Ok(())
    }
}

/// Load configuration groups from file
fn load_config_file() -> anyhow::Result<HashMap<String, UserConfig>> {
    log::debug!("Loading configuration groups from file");
    let config_path = utils::get_config_path()?;

    if !config_path.exists() {
        log::debug!("Configuration file does not exist");
        return Ok(HashMap::new());
    }

    let content = fs::read_to_string(&config_path)?;
    let config_file: ConfigFile = serde_json::from_str(&content)?;
    log::debug!("Successfully loaded {} configuration groups", config_file.groups.len());

    Ok(config_file.groups)
}

/// Batch get git user configuration
///
/// Uses single git command to get name and email, avoiding multiple calls
fn get_git_user_batch(global: bool) -> anyhow::Result<UserConfig> {
    let scope = if global { "--global" } else { "--local" };
    log::debug!("Batch fetching git user configuration ({})", scope);

    let output = Command::new("git")
        .args(["config", scope, "--get-regexp", "^user\\.(name|email)$"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::format_err!("Failed to get git configuration: {}", scope));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut name = String::new();
    let mut email = String::new();

    for line in stdout.lines() {
        if let Some((key, value)) = line.split_once(' ') {
            match key {
                "user.name" => name = value.to_string(),
                "user.email" => email = value.to_string(),
                _ => {}
            }
        }
    }

    if name.is_empty() && email.is_empty() {
        return Err(anyhow::anyhow!("Git user configuration is empty"));
    }

    log::debug!("Retrieved user configuration: {} <{}>", name, email);
    Ok(UserConfig { name, email })
}

/// Set git user configuration
pub fn set_git_user(user: &UserConfig, global: bool) -> anyhow::Result<()> {
    let scope = if global { "--global" } else { "--local" };
    log::debug!(
        "Setting git user configuration ({}): {} <{}>",
        scope,
        user.name,
        user.email
    );

    // Set name
    let status = Command::new("git")
        .args(["config", scope, "user.name", &user.name])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to set git user.name"));
    }

    // Set email
    let status = Command::new("git")
        .args(["config", scope, "user.email", &user.email])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to set git user.email"));
    }

    log::debug!("Git user configuration set successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert!(config.groups.is_empty());
        assert!(config.global_user.is_none());
        assert!(config.project_user.is_none());
    }

    #[test]
    fn test_user_config_serialization() {
        let user = UserConfig {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: UserConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Test User");
        assert_eq!(deserialized.email, "test@example.com");
    }
}
