//! # Utility Functions Module
//!
//! Provides various utility functions needed by the application,
//! including path handling, git repository detection, and colored output.
//!
//! ## Main Features
//! - Get configuration file path
//! - Check if current directory is a git repository
//! - Colored console output

use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;

/// Get configuration file path
///
/// Returns configuration file path based on operating system:
/// - Linux/macOS: $XDG_CONFIG_HOME/gum/config.jsonc (default: ~/.config/gum/config.jsonc)
/// - Windows: %APPDATA%\gum\config.jsonc
///
/// # Returns
/// - `Ok(PathBuf)`: Full path to configuration file
/// - `Err`: Error when unable to get configuration directory
pub fn get_config_path() -> anyhow::Result<PathBuf> {
    log::debug!("Getting config path");

    #[cfg(windows)]
    {
        let appdata =
            std::env::var("APPDATA").context("Could not find APPDATA environment variable")?;
        let config_path = PathBuf::from(appdata).join("gum").join("config.jsonc");
        log::debug!("Config path: {:?}", config_path);
        Ok(config_path)
    }

    #[cfg(not(windows))]
    {
        let xdg_config_home = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = home_dir().ok_or("Could not find home directory").unwrap();
            home.join(".config").to_string_lossy().to_string()
        });
        let config_path = PathBuf::from(xdg_config_home)
            .join("gum")
            .join("config.jsonc");
        log::debug!("Config path: {:?}", config_path);
        Ok(config_path)
    }
}

pub fn is_git_repository() -> bool {
    log::debug!("Checking if current directory is a git repository");
    let result = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|output| !output.stdout.is_empty())
        .unwrap_or(false);
    log::debug!("Is git repository: {}", result);
    result
}

/// Colored print function
///
/// Uses ANSI escape sequences to output colored text to console. Supported colors
/// include red, yellow, green, cyan, white. If unsupported color is specified,
/// defaults to white.
///
/// # Parameters
/// - `val`: Text content to print
/// - `color`: Color name
pub fn printer(val: &str, color: &str) {
    let color_code = match color {
        "red" => "\x1b[31m",
        "yellow" => "\x1b[33m",
        "green" => "\x1b[32m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        "blue" => "\x1b[34m",
        _ => "\x1b[37m",
    };

    println!();
    println!("{}{}\x1b[0m", color_code, val);
}

/// Colored print function (no newline)
///
/// Uses ANSI escape sequences to output colored text to console. Supported colors
/// include red, yellow, green, cyan, white. If unsupported color is specified,
/// defaults to white.
///
/// # Parameters
/// - `val`: Text content to print
/// - `color`: Color name
pub fn printer_no_newline(val: &str, color: &str) {
    let color_code = match color {
        "red" => "\x1b[31m",
        "yellow" => "\x1b[33m",
        "green" => "\x1b[32m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        "blue" => "\x1b[34m",
        _ => "\x1b[37m",
    };

    print!("{}{}\x1b[0m", color_code, val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_path() {
        let path = get_config_path().unwrap();
        assert!(path.ends_with("config.jsonc"));
    }

    #[test]
    fn test_printer() {
        // Just test that it doesn't panic
        printer("test", "red");
        printer("test", "invalid");
    }
}
