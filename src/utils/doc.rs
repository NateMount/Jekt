// [Utils: Doc]

/// Displays detailed documentation on the requested command
/// 
/// **Params**
/// - `command` : name of command to get more info on
/// 
/// **Considerations**
/// - If command does not exist then inform the user and exit
pub fn doc(command: String){
    match command.to_ascii_lowercase().as_str() {
        "list" | "ls" => list_doc(),
        "clear" | "cl" => clear_doc(),
        "info" | "inf" => info_doc(),
        "todo" => todo_doc(),
        "new" | "add" => new_doc(),
        "path" | "src" => path_doc(),
        "delete" | "del" | "rm" => delete_doc(),
        "restore" | "revive" => restore_doc(),
        "archive" => archive_doc(),
        "set" => set_doc(),
        "pop" => pop_doc(),
        _ => println!("\x1b[1;33m[%]\x1b[0m Unrecognized command {}", command)
    }
}

/// Documentation for list command
fn list_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m List | Ls \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt list

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe list command is used to list all projects stored in the active index ( \x1b[3mArchive projects will not be listed, to do so use the command: \x1b[1;35mArchive\x1b[0m )
\tEach project will display its \x1b[1;34mProjectId\x1b[0m & \x1b[1;34mProjectDescription\x1b[0m line by line.
    ")
}

/// Documentation for clear command
fn clear_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Clear | Cl \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt clear

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe clear command will permenantly remove all projects the \x1b[1;32mActive Index\x1b[0m & \x1b[1;32mArchive Store\x1b[0m. This command is primarily intended for
\tdebugging purposes but can be used by the user for an easy reset.
    ")
}

/// Documentation for info command
fn info_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Info \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt info \x1b[1;34m<project_id>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe info command is used to display all information about an indexed project. The info command will not display any information about archived projects
\tuntil they are restored.
    ")
}

/// Documentation on new command
fn new_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m New | Add \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt new \x1b[1;34m<project_id>\x1b[0m \x1b[1;34m<project_path>\x1b[0m \x1b[1;34m<project_description>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe new/add command is used to add a new project to the \x1b[1;32mActive Index\x1b[0m. When a new project is created it will have a unique id provided by the user 
\tas well as a project root path and description. When created a project is automatically populated with a \x1b[3;32mStart-Date\x1b[0m and a \x1b[3;32mStatus\x1b[0m of 'New'. New 
\tprojects are populated to the end of the \x1b[1;32mActive Index\x1b[0m.

\x1b[1;34m[\x1b[0m Command Considerations \x1b[1;34m]\x1b[0m
\t\x1b[1;34m@\x1b[0m If no arguments are provided a help snippet will be displayed instead of making a new project.
\t\x1b[1;34m@\x1b[0m If no path is provided the program will default to the current working directory.
\t\x1b[1;34m@\x1b[0m If no description is provided the description will simply be blank
\t\x1b[1;34m@\x1b[0m If a project already exists with the provided project_id, the program will not make a new project ( \x1b[3;35mNote\x1b[0m\x1b[3m: ProjectId's are not case-sensitive \x1b[0m)
    ")
}

/// Documentation for path command
fn path_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Path | Src \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt path \x1b[1;34m<project_id>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe path command outputs the path to the project root of the specified project, if it exists. If the project does not exist the program will inform
\tthe user and then exit.
    ")
}

/// Documentation for delete command
fn delete_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Delete | Del | Src \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt delete \x1b[1;34m<project_id>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe delete command is used to delete a project from the \x1b[1;32mActive Index\x1b[0m if it exists.

\x1b[1;34m[\x1b[0m Command Considerations \x1b[1;34m]\x1b[0m
\t\x1b[1;34m@\x1b[0m If no arguments are provided a help snippet will be displayed instead of attempting a delete.
    ")
}

/// Documentation for archive command
fn archive_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Archive \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt archive \x1b[1;34m<project_id>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe archive command is used to move a project from the \x1b[1;32mActive Index\x1b[0m to the \x1b[1;32mArchive Store\x1b[0m or to
\tlist all currently archived projects.

\x1b[1;34m[\x1b[0m Command Considerations \x1b[1;34m]\x1b[0m
\t\x1b[1;34m@\x1b[0m If no arguments are provided all projects in archive will be displayed similar to \x1b[1;34mlist\x1b[0m command
\t\x1b[1;34m@\x1b[0m A project moved to the archive cannot be interacted with outside of the \x1b[1;34marchive\x1b[0m & \x1b[1;34mrestore\x1b[0m commands
    ")
}

/// Documentation for restore command
fn restore_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Restore \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt restore \x1b[1;34m<project_id>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe restore command is used to return a project from the \x1b[1;32mArchive Store\x1b[0m to the \x1b[1;32mActive Index\x1b[0m
    ")
}

/// Documentation for the set command
fn set_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Set \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt set \x1b[1;34m<project_id>\x1b[0m \x1b[1;34m<project_key>\x1b[0m \x1b[1;34m<value>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe set command is used to update attribute values for projects. 

\x1b[1;34m[\x1b[0m Command Considerations \x1b[1;34m]\x1b[0m
\t\x1b[1;34m@\x1b[0m If no value is provided the project attribute will be set to an empty string.
\t\x1b[1;34m@\x1b[0m If no key is provided, correct attributes will be displayed along with their 
\t  descriptions, then the program will exit.
    ")
}

/// Documentation for the pop command
fn pop_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Pop \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt pop \x1b[1;34m<project_id>\x1b[0m \x1b[1;34m<project_key>\x1b[0m \x1b[1;34m<value>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe pop command is used to remove elements from project attributes. If popping from a single value attribute the attribute will be 
\tset to an empty string and will not require a 'value' parameter. If you want to pop a value from an array attribute the value will
\tbe required.
    ")
}

/// Documentation for clear command
fn todo_doc(){
    println!("
\x1b[1;34m[\x1b[0m\x1b[3m Todo \x1b[1;34m]\x1b[0m:

\t\x1b[1;32mUsage\x1b[0m: jekt todo \x1b[1;34m<project_id>\x1b[0m \x1b[1;34m<todo_description>\x1b[0m

\x1b[1;34m[\x1b[0m Command Description \x1b[1;34m]\x1b[0m
\tThe todo command is a quick alias for the \x1b[1;32mset\x1b[0m command with the identifier key pre-loaded to \x1b[3;34m'todo'\x1b[0m.
    ")
}