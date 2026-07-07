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
    stack: Vec<String>,
    tags: Vec<String>,
    path: String,
    state: String,
    start_date: toml::value::Datetime,
}

#[derive(Debug, Deserialize)]
struct ProjectIndex {
    project: Vec<Project>
}

// === Functions

fn load_projects() -> ProjectIndex {
    let index_data = fs::read_to_string(INDEX_PATH)
    .expect(&format!("\x1b[1;31m[!]\x1b[0m Jekt-Index file cannot be accessed \x1b[3;34m@{}\x1b[0m\n", INDEX_PATH));

    toml::from_str(&index_data)
    .expect("\x1b[1;31m[!]\x1b[0m Jekt-Index file cannot be parsed\n")
}

// === Command Functions

pub fn list(){
    let index: ProjectIndex = load_projects();

    println!("\t\x1b[1;4;34mProjectId\x1b[0m\t\x1b[1;4mDescription\x1b[0m");
    for project in index.project {
        println!(">>\t\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m\t\x1b[3m{}\x1b[0m", project.id, project.desc);
    }
}

pub fn info(project_id: String){
    let index: ProjectIndex = load_projects();

    for project in index.project{
        if project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase() {
            println!("\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m: \x1b[1;32m( STARTED\x1b[0m {} \x1b[1;32m)\x1b[0m", project.id, project.start_date);
            println!("\t\x1b[1;35m[\x1b[0m \x1b[4mDescription\x1b[0m \x1b[1;35m]\x1b[0m: \x1b[3m{}\x1b[0m", project.desc);
            println!("\t\x1b[1;34m(\x1b[0m Path  \x1b[1;34m)\x1b[0m: {}", project.path);
            println!("\t\x1b[1;34m(\x1b[0m State \x1b[1;34m)\x1b[0m: {}", project.state);
            println!("\t\x1b[1;34m(\x1b[0m Stack \x1b[1;34m)\x1b[0m: {:?}", project.stack);
            println!("\t\x1b[1;34m(\x1b[0m Tags  \x1b[1;34m)\x1b[0m: {:?}", project.tags);
        }
    }

}