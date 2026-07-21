#! /bin/bash

# Basic installer script for Jekt

printf "\033[1;34m[*]\033[0m Starting Installation\n"

# Checking if cargo is installed
printf "\033[1;3mCargo\033[0m: "
if command -v cargo &> /dev/null; then 
    printf "\033[1;32mDetected\033[0m\n"
else
    printf "\033[1;31mNot Detected\033[0m ( Install Cargo to Continue )\n\n\033[1;35mLink\033[0m: \033[4mhttps://doc.rust-lang.org/cargo/getting-started/installation.html\033[0m\n\n"
    exit
fi

printf "\033[1;35m[i]\033[0m Would You Like to Set Custom Locations For Index & Archive ( \033[1;3mDefault is /opt/jekt\033[0m ) [y/N] "
read -r -p "" use_custom_loc

if [[ $use_custom_loc == "y" ]]; then 
    printf "\033[1;35m[i]\033[0m Root Path for Index & Archive: "
    read -r -p "" new_loc

    mkdir -p $new_loc;
    touch $new_loc/jekt-index.toml
    touch $new_loc/jekt-archive.toml
    sed -i "s#/opt/jekt/#$new_loc/#g" src/commands.rs
else 
    printf "\033[1;34m[*]\033[0m Adding Index & Archive to \033[1;34m/opt/jekt/\033[0m\n"
    sudo mkdir -p /opt/jekt;
    sudo chown $USER /opt/jekt
    touch /opt/jekt/jekt-index.toml
    touch /opt/jekt/jekt-archive.toml  
fi

printf "\033[1;34m[*]\033[0m Index & Archive Built Successfully\n"

printf "\n\033[1;34m[*]\033[0m Building Program\n"

cargo build --release

printf "\n\033[1;34m[*]\033[0m Moving Binary to \033[1;34m/usr/local/bin\033[0m\n"
sudo cp target/release/jekt /usr/local/bin/jekt
sudo chmod +x /usr/local/bin/jekt