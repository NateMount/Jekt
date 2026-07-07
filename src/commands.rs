// [Jekt:Commands]

// === Imports

use std::fs;
use serde::Deserialize;

// === Constants
// TODO: Replace with full install path
const INDEX_PATH: &str  = "./resources/jekt-index.toml";
const CONFIG_PATH: &str = "./resources/jekt-conf.toml";

// Structs

#[derive(Debug, Deserialize)]
struct Project {
    id: String,
    desc: String,
    stack: Vec<String>
}

#[derive(Debug, Deserialize)]
struct ProjectIndex {
    project: Vec<Project>
}

// === Command Functions

pub fn list(){

    let index_data = fs::read_to_string(INDEX_PATH)
    .expect(&format!("\x1b[1;31m[!]\x1b[0m Jekt-Index file cannot be accessed \x1b[3;34m@{}\x1b[0m\n", INDEX_PATH));

    let index: ProjectIndex = toml::from_str(&index_data)
    .expect("\x1b[1;31m[!]\x1b[0m Jekt-Index file cannot be parsed\n");

    println!("\t\x1b[1;4;34mProjectId\x1b[0m\t\x1b[1;4mDescription\x1b[0m\t\t\x1b[1;4;35mStack\x1b[0m");
    for project in index.project {
        println!(">>\t\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m\t{}\t\x1b[1;35m{:?}\x1b[0m", project.id, project.desc, project.stack);
    }

}