# Jekt

<div align="center">
  <img src="https://img.shields.io/badge/Language-Rust-ea4a35?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Package%20Manager-Cargo-8ab4f8?style=for-the-badge" alt="Cargo" />
  <img src="https://img.shields.io/badge/Status-Initial%20Release-orange?style=for-the-badge" alt="Status" />
</div>

<br />

**Jekt** is a lightweight project manager and workspace indexer written from scratch in Rust. 

It is designed specifically for developers, hackers, and builders who frequently jump between multiple codebases, manage numerous concurrent projects, or find themselves constantly interrupted and pulled away from their active workflows. 

---

## The Problem

Context-switching is expensive. When managing a graveyard of half-finished repositories, experimental configurations, and client projects, the friction of remembering where you left off, what environment variables were required, or what state a service was in kills momentum. 

**Jekt solves this** by acting as your terminal-native command center. Track project metadata, manage embedded TO-DO lists, organize tech stacks, and find any codebase instantly.

---
## Features

* **Centralized Workspace Indexing:** Keep track of project paths, descriptions, tech stacks, and custom tags.
* **Inline Task & State Tracking:** Attach states (*e.g., Active, Paused, Testing*) and maintain project-specific TODO lists directly inside the CLI.
* **Instant Project Identification (`which`):** Check if your current terminal path belongs to a tracked workspace.
* **Archiving System:** Move inactive projects to cold storage to keep your active index clean without losing history.
* **Fast Tag & ID Search:** Find relevant projects using partial matches or tags.
---

### Commands

| Command | Aliases | Parameters | Description |
| :--- | :--- | :--- | :--- |
| `list` | `ls` | None | List all active projects in the index. |
| `new` | `add` | `<projectId> [path] [description]` | Track a new project. Defaults to current path `.` if omitted. |
| `info` | `inf` | `<projectId>` | Display detailed metadata, stack, tags, and TODOs for a project. |
| `which` | `?` | `[path]` | Detects which project corresponds to the current working directory. |
| `path` | `src` | `<projectId>` | Output the root directory path of a project. |
| `search` | `find` | `<query>` | Search active projects by ID or tags. |
| `set` || `<projectId> <key> <value>` | Set or append project metadata. Key options: `desc`, `stack`, `tag`, `path`, `state`, `todo`. |
| `pop` || `<projectId> <key> [value]` | Remove a value or clear an attribute field. |
| `todo` || `<projectId> [task]` | Shortcut to add or inspect tasks for a given project. |
| `tag` || `<projectId> [tag]` | Shortcut to manage tags for a given project. |
| `archive` | | `[projectId]` | Move a project to the archive index. If no ID is passed, lists all archived projects. |
| `restore` | `revive` | `<projectId>` | Restore an archived project back to the active index. |
| `delete` | `del`, `rm` | `<projectId>` | Remove a project from the indexer (does **not** delete actual files). |
| `clear` | `cl` | None | Reset both the active index and the archive TOML files. |
| `version` | `v` | None | Output current Jekt CLI version. |
| `debug` | `dbg` | `<index\|archive>` | Print internal file storage paths for debugging. |
| `doc` | |`<command>` | Show documentation for a specific jekt command. |
---

## Installation & Setup

### Prerequisites

* [Rust & Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system.
* Linux-Based OS

### Running the Installer
Run the included install script to configure Jekt on your system:

```bash
chmod +x install.sh
./install.sh
```

### The installer will:
* Verify **cargo** is installed on your system.
* Setup the TOML data stores at `/opt/jekt/` ( *or allow you to specify a custom directory* ).
* Build the binary in release mode using cargo build --release.
* Copy the compiled binary to `/usr/local/bin/jekt` for global terminal access.

## Storage Architecture
Jekt persists data using structured TOML files:
* **Active Index**: `/opt/jekt/jekt-index.toml`
* **Archive Store**: `/opt/jekt/jekt-archive.toml` *used for cold storage*

***Note***: If custom storage paths were configured during execution of `install.sh`, `src/commands.rs` will be automatically updated with your new target paths prior to compilation.

## Example run

```bash
# Register current workspace
jekt new my-api . "REST API microservice built with Axum"

# Set metadata and state
jekt set my-api state "In Development"
jekt set my-api stack "Rust"
jekt set my-api stack "Tokio"

# Add tasks
jekt set my-api todo "Implement JWT authentication"
jekt set my-api todo "Setup Docker containerization"

# Inspect the workspace
jekt info my-api

# Move project into archive
jekt archive my-api

# Restore project
jekt restore my-api

# Find which project you are currently working in
jekt which
```