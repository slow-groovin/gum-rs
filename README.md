# Git User Manager (gum-rs)

A Rust implementation of the [gum](https://github.com/gauseen/gum) Git multiple user config manager.

- Compatible with `gum` usage
- 10x faster on low-spec machines

## Overview

This is a command-line tool for managing multiple Git user configurations. It allows developers to easily switch Git user identities.


## Installation

See the steps on the [releases](https://github.com/slow-groovin/gum-rs/releases) page.

## Usage

### List all user config groups

```bash
gum list
```

Output example:
```
Currently used name=gauseen email=gauseen@gmail.com
┌────────────┬─────────┬─────────────────────────┐
│ group-name │    name │                   email │
├────────────┼─────────┼─────────────────────────┤
│    global  │ gauseen │ gauseen@gmail.com       │
│    user1   │ li si   │ lisi@gmail.com          │
│    user2   │ wang er │ wanger@gmail.com        │
└────────────┴─────────┴─────────────────────────┘
```

### Set user config group

```bash
# Set user config group
gum set user1 --name "li si" --email "lisi@gmail.com"

# Set user config group (provide only one parameter)
gum set user2 --email "wanger@gmail.com"
```

### Use user config group

```bash
# Use specified config in current Git repository
gum use user1

# Use specified config in global Git configuration
gum use user1 --global
```

Output example:
```
Currently used name=li si email=lisi@gmail.com
```

### Delete user config group

```bash
gum delete user1
```

## Command Reference

```bash
Usage: gum [options] [command]

Options:
  -V, --version               output the version number
  -h, --help                  display help for command

Commands:
  list                        List all the user config group
  set [options] <group-name>  Set one group for user config
    --name                    User name
    --email                   User email
  use [options] <group-name>  Use one group name for user config
    --global                  Git global config
  delete <group-name>         Delete one group
  help [command]              display help for command
```

## Configuration File
> It's different from `gum`

Configuration file location:
- Linux/macOS: `$XDG_CONFIG_HOME/gum/config.jsonc` (default: `~/.config/gum/config.jsonc`)
- Windows: `%APPDATA%\gum\config.jsonc`

Configuration file uses JSONC format:

```jsonc
{
  "groups": {
    "user1": {
      "name": "li si",
      "email": "lisi@gmail.com"
    },
    "user2": {
      "name": "wang er",
      "email": "wanger@gmail.com"
    }
  }
}
```


## Debug Log
```sh
export RUST_LOG=debug
```

## Development

```bash
# Build project
cargo build

# Run tests
cargo test

# Run in development mode
cargo run -- --help

# Release build
cargo build --release
```
