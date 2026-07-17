// [Jekt:Commands]

// === Imports
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::utils::fileops::{load_source, write_project, blank_source};

// === Constants
// TODO: Replace with full install path
const INDEX_PATH: &str = "/opt/jekt/jekt-index.toml";
const ARCHIVE_PATH: &str = "/opt/jekt/jekt-archive.toml";

// === Structs

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    id: String,
    desc: String,
    stack: Vec<String>,
    tags: Vec<String>,
    path: String,
    state: String,
    start_date: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectIndex {
    pub project: Vec<Project>
}

// === Command Functions

pub fn list(){
    let index: ProjectIndex = load_source(INDEX_PATH);

    if index.project.is_empty() {
        println!("\x1b[1;32m[#]\x1b[0m You currently have no projects, use: \x1b[3;1mjekt new \x1b[3;34m`projectId` `path` `description`\x1b[0m");
    } else {
        println!("\t\x1b[1;4;34mProjectId\x1b[0m\t\x1b[1;4mDescription\x1b[0m");
        for project in index.project {
            println!(">>\t\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m\t\x1b[3m{}\x1b[0m", project.id, project.desc);
        }
    }
}

pub fn clear() {

    println!("\x1b[1;32m[#]\x1b[0m Clearing sources");
    blank_source(INDEX_PATH);
    blank_source(ARCHIVE_PATH);

}

pub fn info(project_id: String){
    
    for project in load_source(INDEX_PATH).project{
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

pub fn new(project_id: String, mut path: String, description: String){
    let index: ProjectIndex = load_source(INDEX_PATH);

    if index.project.iter().any(|project| project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase() ) {
        println!("\x1b[1;31m[!]\x1b[0m Project with name \x1b[3;34m`{}`\x1b[0m already exists, cannot add project", project_id);
    } else {
        println!("\x1b[1;32m[#]\x1b[0m Creating project \x1b[3;34m`{}`\x1b[0m", project_id);
        
        if path == String::from("."){
            let cwd = std::env::current_dir().expect("");
            path = cwd.into_os_string().into_string().unwrap_or(String::from("?"));
        }

        match write_project( vec![Project {
            id: project_id, desc: description, 
            stack: vec![], tags: vec![], 
            path: path, 
            state: String::from("New"), 
            start_date: Utc::now().to_rfc3339()
        }], INDEX_PATH) {
            Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Project added to index"),
            Err(error) => println!("\x1b[1;31m[!]\x1b[0m Error in building new project:\n{}\n", error)
        }
    }
}

pub fn path(project_id: String){

    for project in load_source(INDEX_PATH).project {
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

    let mut index: ProjectIndex = load_source(INDEX_PATH);

    for (idx, project) in index.project.iter().enumerate() {
        if project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase() {

            println!("\x1b[1;32m[#]\x1b[0m Deleting \x1b[3;34m`{}`\x1b[0m", project_id);
            index.project.remove(idx);

            blank_source(INDEX_PATH);
            match write_project(index.project, INDEX_PATH){
                Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Removed project and updated index"),
                Err(_) => println!("\x1b[1;31m[!]\x1b[0m Could not save project")
            }

            return;
        }
    }

    index = load_source(ARCHIVE_PATH);

    for (idx, project) in index.project.iter().enumerate() {
        if project.id.to_ascii_lowercase() == project_id.to_ascii_lowercase() {

            println!("\x1b[1;32m[#]\x1b[0m Deleting \x1b[3;34m`{}`\x1b[0m", project_id);
            index.project.remove(idx);

            blank_source(ARCHIVE_PATH);
            match write_project(index.project, ARCHIVE_PATH){
                Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Removed project and updated index"),
                Err(_) => println!("\x1b[1;31m[!]\x1b[0m Could not save project")
            }

            return;
        }
    }

    println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);
}

pub fn archive(project_id: String){

    if project_id == "_na" {
        let index: ProjectIndex = load_source(ARCHIVE_PATH);

        if index.project.is_empty() { println!("\x1b[1;32m[#]\x1b[0m You currently have no archived projects"); }
        else {
            println!("\t\x1b[1;4;34mProjectId\x1b[0m\t\x1b[1;4mDescription\x1b[0m");
            for project in index.project {
                println!(">>\t\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m\t\x1b[3m{}\x1b[0m", project.id, project.desc);
            }
        }

    } else {

        let mut index:ProjectIndex = load_source(INDEX_PATH);

        for (idx, project) in index.project.iter().enumerate() {
            if project_id.to_ascii_lowercase() == project.id.to_ascii_lowercase() {

                let swap:Project = index.project.remove(idx);

                blank_source(INDEX_PATH);
                match write_project(index.project, INDEX_PATH){
                    Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Removed project and updated index"),
                    Err(_) => println!("\x1b[1;31m[!]\x1b[0m Could not save project")
                }

                match write_project(vec![swap], ARCHIVE_PATH){
                    Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Moved project into archive"),
                    Err(_) => println!("\x1b[1;31m[!]\x1b[0m Could not move project")
                }

                return;        
            }
        }

        println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);
    }
}

pub fn restore(project_id: String){

    if project_id == "_na" { 
        println!("\x1b[1;33m[%]\x1b[0m The resore command is used to move a project from your archive to active index ");
    } else {
        
        let mut index:ProjectIndex = load_source(ARCHIVE_PATH);

        for (idx, project) in index.project.iter().enumerate() {
            if project_id.to_ascii_lowercase() == project.id.to_ascii_lowercase() {

                println!("\x1b[1;32m[#]\x1b[0m Restoring \x1b[1;34m`{}`\x1b[0m", project_id);
                
                let swap:Project = index.project.remove(idx);

                blank_source(ARCHIVE_PATH);
                match write_project(index.project, ARCHIVE_PATH) {
                    Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Removed project and updated archive"),
                    Err(_) => println!("\x1b[1;31m[!]\x1b[0m Could not save project")
                }

                match write_project(vec![swap], INDEX_PATH) {
                    Ok(_) => println!("\x1b[1;32m[#]\x1b[0m Updated index with restored project"),
                    Err(_) => println!("\x1b[1;31m[!]\x1b[0m Could not save project")
                }

                return;
            }
        }

        println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);
    }
}

pub fn set(project_id:String, key:String, value: String){

    if project_id == String::from("_na") {
        println!("\x1b[1;33m[%]\x1b[0m Usage: jekt update \x1b[3;34m`projectId`\x1b[0m \x1b[3;34m`key`\x1b[0m \x1b[3;34m`value`\x1b[0m");
        return;
    }

    if key == String::from("_na") {
        println!("\x1b[1;33m[%]\x1b[0m Potential project keys include:");
        for key_inf in vec![
            (" Desc  ", "Description for project"), 
            (" Stack ", "List of technology / tools / languages used in project"),
            (" Tag   ", "Tag to be used to describe project or aid in searching for project"),
            (" Path  ", "File path to root of project"),
            (" State ", "User-defined state of project"),
        ]{ println!("  (\x1b[1;33m{}\x1b[0m):\t{}", key_inf.0, key_inf.1) }
        return;
    }

    let mut index: ProjectIndex = load_source(INDEX_PATH);

    for (idx, project) in index.project.iter().enumerate() {
        if project_id.to_ascii_lowercase() == project.id.to_ascii_lowercase() {

            let mut active: Project = index.project.remove(idx);

            blank_source(INDEX_PATH);
            write_project(index.project, INDEX_PATH).expect("");

            match key.to_ascii_lowercase().as_str() {
                "desc" | "description" => active.desc = value,
                "stack" => active.stack.push(value),
                "tag" | "tags" => active.tags.push(value),
                "path" => active.path = value,
                "state" | "status" => active.state = value,
                _ => println!("\x1b[1;31m[!]\x1b[0m Unrecognized key!")
            }

            write_project(vec![active], INDEX_PATH).expect("");

            return;
        }
    }

    println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);
}

pub fn pop (project_id: String, key: String, value: String){

    if project_id == String::from("_na") {
        println!("\x1b[1;33m[%]\x1b[0m Pop command is used to pop a value from a specified project, thereby removing it");
        return;
    }

    if key == String::from("_na") {
        println!("\x1b[1;33m[%]\x1b[0m Potential project keys to pop:");
        for key_inf in vec![
            (" Desc  ", "Description for project"), 
            (" Stack ", "List of technology / tools / languages used in project"),
            (" Tag   ", "Tag to be used to describe project or aid in searching for project"),
            (" Path  ", "File path to root of project"),
            (" State ", "User-defined state of project"),
        ]{ println!("  (\x1b[1;33m{}\x1b[0m):\t{}", key_inf.0, key_inf.1) }
        return;
    }

    let mut index: ProjectIndex = load_source(INDEX_PATH);

    for (idx, project) in index.project.iter().enumerate() {
        if project_id.to_ascii_lowercase() == project.id.to_ascii_lowercase() {

            let mut active: Project = index.project.remove(idx);

            blank_source(INDEX_PATH);
            write_project(index.project, INDEX_PATH).expect("");

            match key.to_ascii_lowercase().as_str() {
                "desc" | "description" => active.desc = String::from(""),
                "stack" => active.stack.retain(|stk| stk.to_ascii_lowercase() != value.to_ascii_lowercase()),
                "tag" | "tags" => active.tags.retain(|tag| tag.to_ascii_lowercase() != value.to_ascii_lowercase()),
                "path" => active.path = String::from(""),
                "state" | "status" => active.state = String::from(""),
                _ => println!("\x1b[1;31m[!]\x1b[0m Unrecognized key!")
            }

            write_project(vec![active], INDEX_PATH).expect("");

            return;
        }
    }

    println!("\x1b[1;33m[%]\x1b[0m Project \x1b[3;34m`{}`\x1b[0m not found", project_id);

}

pub fn search(key: String){

    for project in load_source(INDEX_PATH).project {
        if project.id.to_ascii_lowercase().contains(&key.to_ascii_lowercase()) || project.tags.iter().any(|tag| tag.to_ascii_lowercase().contains(&key.to_ascii_lowercase())) {
            println!("\x1b[1;34m[\x1b[0m {} \x1b[1;34m]\x1b[0m: \x1b[1;32m( STARTED\x1b[0m {} \x1b[1;32m)\x1b[0m", project.id, project.start_date);
            println!("\t\x1b[1;35m[\x1b[0m \x1b[4mDescription\x1b[0m \x1b[1;35m]\x1b[0m: \x1b[3m{}\x1b[0m", project.desc);
            println!("\t\x1b[1;34m(\x1b[0m Path  \x1b[1;34m)\x1b[0m: {}", project.path);
            println!("\t\x1b[1;34m(\x1b[0m State \x1b[1;34m)\x1b[0m: {}", project.state);
            println!("\t\x1b[1;34m(\x1b[0m Stack \x1b[1;34m)\x1b[0m: {:?}", project.stack);
            println!("\t\x1b[1;34m(\x1b[0m Tags  \x1b[1;34m)\x1b[0m: {:?}", project.tags);
        }
    }

}