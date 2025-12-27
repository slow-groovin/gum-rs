//! Application entry point
//!
//! Responsible for parsing command line arguments and dispatching to corresponding handlers.
//! Supports listing, setting, using, and deleting Git user configuration groups.

use clap::Parser;
use env_logger::Builder;
use gum_rs::cli::{Cli, Commands};
use gum_rs::config::{Config, UserConfig};
use gum_rs::utils;
use std::collections::HashMap;
use std::io::Write;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    Builder::from_env(env_logger::Env::default())
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                buf.timestamp_micros(),
                record.level(),
                record.args()
            )
        })
        .init();

    log::debug!("Starting gum application");

    let cli = Cli::parse();
    log::debug!("Parsed CLI command: {:?}", cli.command);

    // Load all configurations at once (parallel execution)
    let mut config = Config::load()?;

    match cli.command {
        Commands::List => handle_list(&config),
        Commands::Set {
            group_name,
            name,
            email,
        } => handle_set(&mut config, group_name, name, email),
        Commands::Use { group_name, global } => handle_use(&mut config, group_name, global),
        Commands::Delete { group_name } => handle_delete(&mut config, group_name),
    }
}

/// Handle list command
fn handle_list(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Executing list command");

    // Use cached configuration directly
    let using = config.get_using_git_user()?;
    utils::printer(
        &format!("Currently using: {} <{}>", using.name, using.email),
        "yellow",
    );
    println!();

    let all_config = config.get_all_config_info();

    if all_config.is_empty() {
        log::info!("No user configuration found");
        println!("No user configuration found.");
        return Ok(());
    }

    log::info!("Displaying {} configuration groups", all_config.len());
    print_config_table(&all_config);

    Ok(())
}

/// Handle set command
fn handle_set(
    config: &mut Config,
    group_name: String,
    name: Option<String>,
    email: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Executing set command, target group: {}", group_name);

    if group_name == "global" {
        log::warn!("Attempting to set reserved group name 'global'");
        utils::printer("Group name cannot be 'global'", "red");
        println!();
        return Err("Group name cannot be 'global'".into());
    }

    if name.is_none() && email.is_none() {
        log::warn!("Set command did not provide username or email");
        utils::printer("Must provide at least one of username or email", "red");
        println!();
        return Err("Must provide at least one of username or email".into());
    }

    // Get existing configuration or create new one
    let mut current_user = config.groups.get(&group_name).cloned().unwrap_or_else(|| {
        log::debug!("Creating new user config for group: {}", group_name);
        UserConfig {
            name: String::new(),
            email: String::new(),
        }
    });

    if let Some(n) = name {
        log::debug!("Setting username: {}", n);
        current_user.name = n;
    }

    if let Some(e) = email {
        log::debug!("Setting email: {}", e);
        current_user.email = e;
    }

    config.groups.insert(group_name.clone(), current_user);
    config.save()?;

    log::info!("Successfully set group: {}", group_name);
    utils::printer(&format!("Successfully set {} group", group_name), "green");
    println!();

    Ok(())
}

/// Handle use command
fn handle_use(
    config: &mut Config,
    group_name: String,
    global: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Executing use command, target group: {} (global: {})", group_name, global);

    let all_config = config.get_all_config_info();
    let user = all_config
        .get(&group_name)
        .ok_or_else(|| format!("{} is an invalid group name", group_name))?;

    // If not global, check if it's a git repository
    if !global && !utils::is_git_repository() {
        log::warn!("Attempting to use local config in non-git directory");
        utils::printer("Current project is not a git repository", "red");
        println!();
        return Err("Current project is not a git repository".into());
    }

    // Set git user configuration
    gum_rs::config::set_git_user(user, global)?;

    // Refresh corresponding cache
    if global {
        config.refresh_global_user()?;
        if let Some(ref global_user) = config.global_user {
            utils::printer(
                &format!("Global use: {} <{}>", global_user.name, global_user.email),
                "green",
            );
        }
    } else {
        config.refresh_project_user()?;
    }

    // Display currently used configuration
    let using = config.get_using_git_user()?;
    utils::printer(
        &format!("Currently using: {} <{}>", using.name, using.email),
        "yellow",
    );

    log::info!("Successfully set git user for group: {}", group_name);
    println!();

    Ok(())
}

/// Handle delete command
fn handle_delete(
    config: &mut Config,
    group_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Executing delete command, target group: {}", group_name);

    if group_name == "global" {
        log::warn!("Attempting to delete reserved group 'global'");
        utils::printer("Cannot delete global", "red");
        println!();
        return Err("Cannot delete global".into());
    }

    if config.groups.remove(&group_name).is_some() {
        config.save()?;
        log::info!("Successfully deleted group: {}", group_name);
        utils::printer(&format!("Successfully deleted {} group", group_name), "green");
        println!();
        Ok(())
    } else {
        log::warn!("Group not found: {}", group_name);
        utils::printer(&format!("{} group not found", group_name), "red");
        println!();
        Err(format!("{} group not found", group_name).into())
    }
}

/// Print configuration table
fn print_config_table(all_config: &HashMap<String, UserConfig>) {
    println!("┌────────────┬─────────┬─────────────────────────┐");
    println!("│ group-name │    name │                   email │");
    println!("├────────────┼─────────┼─────────────────────────┤");

    for (group_name, user) in all_config {
        let group_name_trunc = truncate_str(group_name, 10);
        let name_trunc = truncate_str(&user.name, 7);
        let email_trunc = truncate_str(&user.email, 23);

        println!(
            "│ {:10} │ {:7} │ {:23} │",
            group_name_trunc, name_trunc, email_trunc
        );
    }

    println!("└────────────┴─────────┴─────────────────────────┘");
}

/// Truncate string
fn truncate_str(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len { s } else { &s[..max_len] }
}
