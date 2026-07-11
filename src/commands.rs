// [Jekt:Commands]

// === Imports

use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use chrono::Utc;

// === Constants
// TODO: Replace with full install path
const INDEX_PATH: &str  = "./resources/jekt-index.toml";
const ARCHIVE_PATH: &str = "./resources/ject-archive.toml";

// === Structs

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    id: String,
    desc: String,
    stack: Vec<String>,
    tags: Vec<String>,
    path: String,
    state: String,
    start_date: String,
}

#[derive(Debug, Deserialize)]
struct ProjectIndex {
    project: Vec<Project>
}

// === Functions

fn load_index(index: &str) -> ProjectIndex {
    let index_data = fs::read_to_string(index)
    .expect(&format!("\x1b[1;31m[!]\x1b[0m Jekt-Index file cannot be accessed \x1b[3;34m@{}\x1b[0m\n", INDEX_PATH));

    match toml::from_str(&index_data) {
        Ok(index) => index,
        Err(_) => ProjectIndex { project: vec![] }
    }
}

fn write_project(project: Project) -> Result<(), std::io::Error> {
    let mut write_out = fs::File::options().append(true).create(true).open(INDEX_PATH)?;
    writeln!(write_out, "\n[[project]]\n{}\n", toml::to_string(&project).expect("\x1b[1;31m[!]\x1b[0m Unable to generate TOML formatted project"))?;
    Ok(())
}

// === Command Functions

pub fn list(){
    let index: ProjectIndex = load_index(INDEX_PATH);

    if index.project.len() == 0 {
        println!("\x1b[1;32m[#]\x1b[0m You currently have no projects, use: \x1b[3;1mjekt new \x1b[3;34m`projectId` `path` `description`\x1b[0m");
    } else {
        println!("\t\x1b[1;4;34mProjectId\x1b[0m\t\x1b[1;4mDescription\x1b[0m");
        for project in index.project {
            println!(">>\t\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m\t\x1b[3m{}\x1b[0m", project.id, project.desc);
        }
    }
}

pub fn info(project_id: String){
    
    for project in load_index(INDEX_PATH).project{
        if project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase(){
            println!("\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m: \x1b[1;32m( STARTED\x1b[0m {} \x1b[1;32m)\x1b[0m", project.id, project.start_date);
            println!("\t\x1b[1;35m[\x1b[0m \x1b[4mDescription\x1b[0m \x1b[1;35m]\x1b[0m: \x1b[3m{}\x1b[0m", project.desc);
            println!("\t\x1b[1;34m(\x1b[0m Path  \x1b[1;34m)\x1b[0m: {}", project.path);
            println!("\t\x1b[1;34m(\x1b[0m State \x1b[1;34m)\x1b[0m: {}", project.state);
            println!("\t\x1b[1;34m(\x1b[0m Stack \x1b[1;34m)\x1b[0m: {:?}", project.stack);
            println!("\t\x1b[1;34m(\x1b[0m Tags  \x1b[1;34m)\x1b[0m: {:?}", project.tags);
            return;
        }
    }

    println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);
}

pub fn new(project_id: String, path: String, description: String){
    let index: ProjectIndex = load_index(INDEX_PATH);

    if index.project.iter().any(|project| project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase() ) {
        println!("\x1b[1;31m[!]\x1b[0m Project with name \x1b[3;34m`{}`\x1b[0m already exists, cannot add project", project_id);
    } else {

        println!("\x1b[1;32m[#]\x1b[0m Creating project \x1b[3;34m`{}`\x1b[0m", project_id);
        
        match write_project( Project {
            id: project_id, desc: description, 
            stack: vec![], tags: vec![], 
            path: path, 
            state: String::from("New"), 
            start_date: Utc::now().to_rfc3339()
        }) {
            Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Project added to index"),
            Err(error) => println!("\x1b[1;31m[!]\x1b[0m Error in building new project:\n{}\n", error)
        }

    }
}

pub fn path(project_id: String){

    for project in load_index(INDEX_PATH).project {
        if project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase() {
            println!("\x1b[1;32m[#]\x1b[0m \x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m @ {}", project.id, project.path);
            return;
        }
    }

    println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);
}

pub fn delete(project_id: String){

    if project_id == String::from("_na") {
        println!("\x1b[1;33m[%]\x1b[0m The delete command is used to remove a project from the indexer \x1b[1;34m(\x1b[0m \x1b[3mNo project content will be deleted\x1b[0m \x1b[1;34m)\x1b[0m");
        return;
    }

    let mut index: ProjectIndex = load_index(INDEX_PATH);

    for (idx, project) in index.project.iter().enumerate() {
        if project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase() {

            println!("\x1b[1;32m[#]\x1b[0m Deleting \x1b[3;34m`{}`\x1b[0m", project_id);
            index.project.remove(idx);

            match fs::File::create(INDEX_PATH){
                Ok(_) => println!("\x1b[1;33m[%]\x1b[0m Saving updated index"),
                Err(_) => println!("\x1b[1;31m[!]\x1b[0m Error saving updated index")
            }

            for keeper in index.project {
                match write_project(keeper){
                    Ok(_) => println!("\x1b[1;33m[#]\x1b[0m Saved project"),
                    Err(_) => println!("\x1b[1;31m[!]\x1b[0m Could not save project")
                }
            }

            return;
        }
    }

    println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);
}

pub fn archive(project_id: String){

    let index: ProjectIndex = load_index(ARCHIVE_PATH);

}

pub fn restore(project_id: String){

    let index: ProjectIndex = load_index(ARCHIVE_PATH);
    
}