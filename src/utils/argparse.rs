// [Utils: Argument Parsing]

use std::env::args;

pub enum RunMode {
    HELP,
    LIST,
    CLEAR,
    INFO(String),
    PATH(String),
    NEW(String, String, String),
    DELETE(String),
    ARCHIVE(String),
    RESTORE(String)
}

pub fn build_run_config() -> RunMode{
    let cli_arg:Vec<String> = args().collect();

    if cli_arg.len() == 1 { return RunMode::HELP }

    match cli_arg[1].to_lowercase().as_str() {
        "list" | "ls" => RunMode::LIST,
        "info" => RunMode::INFO(
            cli_arg.get(2).unwrap_or(&String::from("*")).clone()
        ),
        "path" | "." => RunMode::PATH(
            cli_arg.get(2).unwrap_or(&String::from("*")).clone()
        ),
        "new" | "+" => RunMode::NEW(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(3).unwrap_or(&String::from(".")).clone(),
            cli_arg.get(4).unwrap_or(&String::from("")).clone()
        ),
        "delete" | "-" => RunMode::DELETE(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "archive" => RunMode::ARCHIVE(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "restore" | "revive" => RunMode::RESTORE(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "clear" | "cl" => RunMode::CLEAR,
        _ => RunMode::HELP
    }
}