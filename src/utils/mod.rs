// [Utils: Generic]

pub mod argparse;
pub mod fileops;

/// Displays program help info
pub fn display_help() {
    println!("
\x1b[1;35m[ Jekt\x1b[0m::Help \x1b[1;35m]\x1b[0m

\x1b[1;34mUsage:\x1b[0m jekt \x1b[1;32m[Command]\x1b[0m [ arg \x1b[1;34m...\x1b[0m ]

\x1b[1;32m[\x1b[0m Commands \x1b[1;32m]\x1b[0m
\t\x1b[1;4;32mcommand\x1b\t\t\x1b[4;34margs\x1b[0m\t\t\t\t\x1b[4mdescritpion\x1b[0m
\t\x1b[1;32mhelp\x1b[0m\t\t\t\t\t\tshow this message
\t\x1b[1;32mversion\x1b[0m\t\t\t\t\t\tshow application version
\t\x1b[1;32mlist\x1b[0m\t\t\t\t\t\tlist all indexed projects
\t\x1b[1;32minfo\x1b[0m\t\t\x1b[34m(\x1b[0m projectId \x1b[34m)\x1b[0m\t\t\tshow metadata and info on \x1b[1;34m`projectId`\x1b[0m
\t\x1b[1;32mpath\x1b[0m\t\t\x1b[34m(\x1b[0m projectId \x1b[34m)\x1b[0m\t\t\tshow full path to \x1b[1;34m`projectId`\x1b[0m
\t\x1b[1;32mnew\x1b[0m\t\t\x1b[34m(\x1b[0m projectId, path \x1b[34m)\x1b[0m\t\tbuild a new project at \x1b[1;34m`path`\x1b[0m with id \x1b[1;34m`projectId`\x1b[0m
\t\x1b[1;32mdelete\x1b[0m\t\t\x1b[34m(\x1b[0m projectId \x1b[34m)\x1b[0m\t\t\tdelete \x1b[1;34m`projectId`\x1b[0m from indexing (\x1b[3;32m Will not delete contents of project \x1b[0m)
\t\x1b[1;32mset\x1b[0m\t\t\x1b[34m(\x1b[0m projectId, key, value \x1b[34m)\x1b[0m\tset \x1b[1;34m`key`\x1b[0m=\x1b[1;34m`value`\x1b[0m for \x1b[1;34m`projectId`\x1b[0m
\t\x1b[1;32mpop\x1b[0m\t\t\x1b[34m(\x1b[0m projectId, key, value \x1b[34m)\x1b[0m\tdrop the \x1b[1;34m`value`\x1b[0m of specified \x1b[1;34m`key`\x1b[0m or just empty \x1b[1;34m`key`\x1b[0m for \x1b[1;34m`projectId`\x1b[0m
\t\x1b[1;32marchive\x1b[0m\t\t\x1b[34m(\x1b[0m projectId \x1b[34m)\x1b[0m\t\t\tmove \x1b[1;34m`projectId`\x1b[0m to the archive (\x1b[3;32m Will not be targetable in commands \x1b[0m)
\t\x1b[1;32mrestore\x1b[0m\t\t\x1b[34m(\x1b[0m projectId \x1b[34m)\x1b[0m\t\t\tmove \x1b[1;34m`projectId`\x1b[0m to the active project heirarchy
    ")
}