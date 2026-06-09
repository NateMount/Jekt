// [Jekt/Main ~]
// Author: Nathan Mount ( @mount )

use std::env;

fn display_help(){
    println!("\
    [:: \x1b[1;36mJekt\x1b[0m Help ::]\n\n\
    \x1b[1;36mUsage:\x1b[0m\n\tjekt \x1b[1;32m[\x1b[0m Command \x1b[1;32m]\x1b[0m \x1b[1;35m( \x1b[0margs=val \x1b[1;35m)\x1b[0m\n\n\
    \x1b[1;32m[\x1b[0m Commands \x1b[1;32m]\x1b[0m\n\
    \t\x1b[1;32mList\x1b[0m\t\t\t Lists all indexed projects
    \t\x1b[1;32mInfo <project>\x1b[0m\t\t Show info on selected project
    \t\x1b[1;32mUpdate <project>\x1b[0m\t Update the \x1b[35m`.jkt`\x1b[0m project info file \x1b[1;36m[! Will make one if not present !]\x1b[0m
    \t\x1b[1;32mNew <path>\x1b[0m\t\t Builds a new \x1b[35m`.jkt`\x1b[0m project info file and adds project to index
    \t\x1b[1;32mDelete <project>\x1b[0m\t Deletes a project from the index
    \t\x1b[1;32mIndex <path>\x1b[0m\t\t Builds a \x1b[35m`.jkt`\x1b[0m for provided path by indexing project
    \n\x1b[1;35m( \x1b[0mArguments\x1b[1;35m )\x1b[0m
    \t\x1b[1;35m/v\x1b[0m\t\t\tVerbose output
    \t\x1b[1;35m/d\x1b[0m\t\t\tDebugging Enabled
    ");
}

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        display_help(); 
        return
    }

}
