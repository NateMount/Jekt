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
        RunMode::INFO(project_id) => commands::info(project_id),
        RunMode::NEW(project_id, path) => commands::new(project_id, path),
        RunMode::PATH(project_id) => commands::path(project_id),
        RunMode::DELETE(project_id) => println!("DELETE::{}", project_id),
        RunMode::ARCHIVE(project_id) => println!("ARCHIVE::{}", project_id)
    }
}