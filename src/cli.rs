//! # CLI Module
//!
//! This module defines the application's command line interface (CLI) structure,
//! using the `clap` library to parse command line arguments.
//! It provides multiple subcommands to manage Git user configuration groups,
//! including listing, setting, using, and deleting configuration groups.
//!
//! ## Main Components
//! - `Cli`: Main CLI struct, contains subcommands.
//! - `Commands`: Subcommand enum, defines all available commands.

use clap::{Parser, Subcommand};

/// Main command line interface struct
///
/// Uses `clap::Parser` derive macro to automatically generate command line
/// argument parsing logic. This struct represents the root command of the
/// application and contains a subcommand field.
#[derive(Parser)]
#[command(name = "gum")]
#[command(about = "Git multiple user config manager")]
#[command(version)]
pub struct Cli {
    /// Subcommand enum, specifies the operation to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommand enum
///
/// Defines all available subcommands, each variant corresponds to a specific operation.
/// Uses `clap::Subcommand` derive macro to generate subcommand parsing logic.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all user configuration groups
    ///
    /// This command displays a list of all currently stored Git user configuration groups.
    /// Each configuration group contains username and email information.
    List,
    /// Set a user configuration group
    ///
    /// Creates or updates a specified user configuration group. Can specify group name,
    /// username, and email. If the group exists, its configuration will be updated;
    /// otherwise a new group will be created.
    Set {
        /// Name of the configuration group, used to identify different user configurations
        group_name: String,
        /// Optional username, if provided will set the username for this group
        #[arg(long)]
        name: Option<String>,
        /// Optional email, if provided will set the email for this group
        #[arg(long)]
        email: Option<String>,
    },
    /// Use specified configuration group
    ///
    /// Applies the specified user configuration group to Git configuration.
    /// Can choose to set it as global or local configuration.
    Use {
        /// Name of the configuration group to use
        group_name: String,
        /// Whether to set as global Git configuration (default is local)
        #[arg(long)]
        global: bool,
    },
    /// Delete specified configuration group
    ///
    /// Deletes the specified user configuration group from storage.
    /// After deletion, the configuration group will no longer be available.
    Delete {
        /// Name of the configuration group to delete
        group_name: String,
    },
}
