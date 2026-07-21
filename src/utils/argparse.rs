// [Utils: Argument Parsing]

use std::env::args;

/// Collection of possible run-modes for the program
/// Each mode defines behavior and parameters for current run
pub enum RunMode {
    HELP,
    LIST,
    CLEAR,
    VERSION,
    DEBUG(String),
    TODO(String, String),
    TAG(String, String),
    WHICH(String),
    INFO(String),
    PATH(String),
    NEW(String, String, String),
    DELETE(String),
    ARCHIVE(String),
    RESTORE(String),
    SET(String, String, String),
    POP(String, String, String),
    SEARCH(String)
}

/// Used to select and populate parameters based on passed in user args
/// 
/// **Considerations**
/// - If no arguments are provided `RunMode:;HELP` is returned
/// - If passed in `command` arg is not matched then `RunMode::HELP` is returned
pub fn build_run_config() -> RunMode{
    let cli_arg:Vec<String> = args().collect();

    if cli_arg.len() == 1 { return RunMode::HELP }

    match cli_arg[1].to_lowercase().as_str() {
        "list" | "ls" => RunMode::LIST,
        "clear" | "cl" => RunMode::CLEAR,
        "version" | "v" => RunMode::VERSION,
        "dbg" | "debug" => RunMode::DEBUG(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "todo" => RunMode::TODO(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(3).unwrap_or(&String::from("")).clone()
        ),
        "tag" | "tags" => RunMode::TAG(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(3).unwrap_or(&String::from("")).clone()
        ),
        "which" | "?" => RunMode::WHICH(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "info" | "inf" => RunMode::INFO(
            cli_arg.get(2).unwrap_or(&String::from("")).clone()
        ),
        "path" | "src" => RunMode::PATH(
            cli_arg.get(2).unwrap_or(&String::from("")).clone()
        ),
        "new" | "add" => RunMode::NEW(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(3).unwrap_or(&String::from(".")).clone(),
            cli_arg.get(4).unwrap_or(&String::from("")).clone()
        ),
        "delete" | "del" | "rm" => RunMode::DELETE(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "archive" => RunMode::ARCHIVE(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "restore" | "revive" => RunMode::RESTORE(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone()
        ),
        "set" => RunMode::SET(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(3).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(4).unwrap_or(&String::from("")).clone()
        ),
        "pop" => RunMode::POP(
            cli_arg.get(2).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(3).unwrap_or(&String::from("_na")).clone(),
            cli_arg.get(4).unwrap_or(&String::from("")).clone()
        ),
        "search" | "find" => RunMode::SEARCH(
            cli_arg.get(2).unwrap_or(&String::from("")).clone()
        ),
        _ => RunMode::HELP
    }
}