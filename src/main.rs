// [Jekt:Main]
// Author(s): Nate Mount (@xxi)

// === Imports
mod utils;
use crate::utils::{argparse::RunMode, display_help};

fn main(){

    let mode: RunMode = utils::argparse::build_run_config();

    match mode {
        RunMode::HELP => display_help(),
        RunMode::LIST => println!("LIST"),
        RunMode::INFO(project_id) => println!("INFO::{}", project_id),
        RunMode::NEW(project_id, path) => println!("NEW::{} @ {}", project_id, path),
        RunMode::PATH(project_id) => println!("PATH::{}", project_id),
        RunMode::DELETE(project_id) => println!("DELETE::{}", project_id),
        RunMode::CONFIG(setting, value) => println!("CONFIG::{}={}", setting, value)
    }
}