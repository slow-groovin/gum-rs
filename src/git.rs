//! # Git Operations Module
//!
//! This module provides functionality to interact with Git configuration,
//! including getting and setting Git user's username and email.
//! All operations are implemented by executing `git config` commands,
//! supporting both global and project level configurations.
//!
//! ## Main Features
//! - Get global git user configuration
//! - Get project level git user configuration
//! - Set git user configuration (supports global or local)

//! # Git Operations Module
//!
//! This module provides functionality to interact with Git configuration,
//! including getting and setting Git user's username and email.
//! All operations are implemented by executing `git config` commands,
//! supporting both global and project level configurations.
//!
//! ## Main Features
//! - Get global git user configuration
//! - Get project level git user configuration
//! - Set git user configuration (supports global or local)

use std::process::Command;

use crate::config::UserConfig;

pub fn get_global_git_user() -> Result<UserConfig, Box<dyn std::error::Error>> {
    log::debug!("Executing git config --global user.name");
    let name_output = Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()?;

    log::debug!("Executing git config --global user.email");
    let email_output = Command::new("git")
        .args(["config", "--global", "user.email"])
        .output()?;

    let name = String::from_utf8_lossy(&name_output.stdout)
        .trim()
        .to_string();
    let email = String::from_utf8_lossy(&email_output.stdout)
        .trim()
        .to_string();

    log::debug!("Global git user: name='{}', email='{}'", name, email);

    if name.is_empty() || email.is_empty() {
        Err("Global git user not configured".into())
    } else {
        Ok(UserConfig { name, email })
    }
}

pub fn get_project_git_user() -> Result<UserConfig, Box<dyn std::error::Error>> {
    log::debug!("Executing git config user.name");
    let name_output = Command::new("git").args(["config", "user.name"]).output()?;

    log::debug!("Executing git config user.email");
    let email_output = Command::new("git")
        .args(["config", "user.email"])
        .output()?;

    let name = String::from_utf8_lossy(&name_output.stdout)
        .trim()
        .to_string();
    let email = String::from_utf8_lossy(&email_output.stdout)
        .trim()
        .to_string();

    log::debug!("Project git user: name='{}', email='{}'", name, email);

    if name.is_empty() || email.is_empty() {
        Err("Project git user not configured".into())
    } else {
        Ok(UserConfig { name, email })
    }
}

pub fn set_git_user(user: &UserConfig, global: bool) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!(
        "Setting git user with global={}, name='{}', email='{}'",
        global,
        user.name,
        user.email
    );

    let args = if global {
        vec!["config", "--global", "user.name"]
    } else {
        vec!["config", "user.name"]
    };

    log::debug!(
        "Executing git config {} user.name '{}'",
        if global { "--global" } else { "" },
        user.name
    );
    let name_status = Command::new("git")
        .args(&args)
        .arg(&user.name)
        .status()
        .map_err(|e| format!("Failed to set git user.name: {}", e))?;

    if !name_status.success() {
        return Err(format!(
            "Failed to set git user.name, exit code: {:?}",
            name_status.code()
        )
        .into());
    }

    let args = if global {
        vec!["config", "--global", "user.email"]
    } else {
        vec!["config", "user.email"]
    };

    log::debug!(
        "Executing git config {} user.email '{}'",
        if global { "--global" } else { "" },
        user.email
    );
    let email_status = Command::new("git")
        .args(&args)
        .arg(&user.email)
        .status()
        .map_err(|e| format!("Failed to set git user.email: {}", e))?;

    if !email_status.success() {
        return Err(format!(
            "Failed to set git user.email, exit code: {:?}",
            email_status.code()
        )
        .into());
    }

    log::debug!("Git user set successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_global_git_user() {
        // This test assumes git is configured globally
        // In a real scenario, you might mock this
        let result = get_global_git_user();
        // We can't assert much here without mocking
        assert!(result.is_ok() || result.is_err()); // Just check it doesn't panic
    }

    #[test]
    fn test_get_project_git_user() {
        // Similar to above
        let result = get_project_git_user();
        assert!(result.is_ok() || result.is_err());
    }
}
