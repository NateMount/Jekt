// [Utils: File Operations]

// === Imports

use crate::commands::{Project,ProjectIndex};
use std::fs;
use std::io::Write;

// === Functions

/// Used to load contents of a toml index file
/// 
/// **Params**
/// - `index` : Path of index to load
/// 
/// **Considerations**
/// - If toml cannot be parsed an empty `ProjectIndex` is returned
/// - If index path cannot be opened an error is returned
pub fn load_source(index: &str) -> ProjectIndex {
    let index_data = fs::read_to_string(index)
    .expect(&format!("\x1b[1;31m[!]\x1b[0m Jekt-Index file cannot be accessed \x1b[3;34m@{}\x1b[0m\n", index));

    match toml::from_str(&index_data) {
        Ok(index) => index,
        Err(_) => ProjectIndex { project: vec![] }
    }
}

/// Writes `ProjectIndex` to index file
/// 
/// **Params**
/// - `projects` : List of projects to be written to index file
/// - `index` : Path of index to write to
pub fn write_project(projects: Vec<Project>, index: &str) -> Result<(), std::io::Error> {
    let mut write_out = fs::File::options().append(true).create(true).open(index)?;

    for project in projects {
        writeln!(
            write_out, 
            "\n[[project]]\n{}\n", 
            toml::to_string(&project).expect("\x1b[1;31m[!]\x1b[0m Unable to generate TOML formatted project")
        )?;
    } Ok(())
}

/// Used to completely clear a file
/// 
/// **Params**
/// - `source` : Path of file to clear
pub fn blank_source(source: &str){
    match fs::File::create(source){
        Ok(_) => println!("\x1b[1;33m[%]\x1b[0m Saving updated index"),
        Err(_) => println!("\x1b[1;31m[!]\x1b[0m Error saving updated index")
    }
}