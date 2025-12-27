//! # Gum - Git User Configuration Manager Library
//!
//! This is a Rust library for managing multiple Git user configurations.
//! Allows users to easily switch between different Git username and email configurations.
//!
//! ## Module Structure
//! - `cli`: Command line interface definition
//! - `config`: Configuration management functionality
//! - `git`: Git configuration operations
//! - `utils`: Utility functions

/// Command line interface module
pub mod cli;
/// Configuration management module
pub mod config;
/// Git operations module
pub mod git;
/// Utility functions module
pub mod utils;