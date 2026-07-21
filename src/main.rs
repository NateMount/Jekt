// [Jekt:Main]
// Author(s): Nate Mount (@xxi)

// === Imports
mod utils;
mod commands;
use crate::utils::{argparse::RunMode, display_help};

fn main(){

    let mode: RunMode = utils::argparse::build_run_config();

    match mode {
        RunMode::HELP => display_help(),
        RunMode::LIST => commands::list(),
        RunMode::CLEAR => commands::clear(),
        RunMode::DEBUG(component) => commands::debug(component),
        RunMode::WHICH(path) => commands::which(path),
        RunMode::VERSION => println!("\x1b[1;32m[#]\x1b[0m \x1b[1;3;34mv0.0\x1b[0m:InDev"),
        RunMode::INFO(project_id) => commands::info(project_id),
        RunMode::NEW(project_id, path, description) => commands::new(project_id, path, description),
        RunMode::PATH(project_id) => commands::path(project_id),
        RunMode::DELETE(project_id) => commands::delete(project_id),
        RunMode::ARCHIVE(project_id) => commands::archive(project_id),
        RunMode::RESTORE(project_id) => commands::restore(project_id),
        RunMode::SET(project_id,key , value) => commands::set(project_id, key, value),
        RunMode::POP(project_id, key, value) => commands::pop(project_id, key, value),
        RunMode::SEARCH(key) => commands::search(key),
        RunMode::TAG(project_id, value) => commands::set(project_id, String::from("tag"), value),
        RunMode::TODO(project_id, value ) => commands::set(project_id, String::from("todo"), value)
    }
}