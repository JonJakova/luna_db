#![allow(dead_code)]

mod luna_action;
mod persistency;
mod storage;

use persistency::persist;
use std::io::Write;
use storage::memory_map;
use storage::storable;
use storage::utils;

fn main() {
    // Print menu on startup
    println!("Welcome to Luna!");
    println!("Type 'help' to see a list of available commands.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    handle_interactive_flow();
}

fn handle_interactive_flow() {
    let mut action: luna_action::Action;
    let mut map = persist::load_from_file(&persist::get_db_file_path());
    loop {
        action = luna_action::get_action();
        match action {
            // add <key> <value> - Adds a new entry to the map
            luna_action::Action::Add { key, value } => {
                map.insert(key, utils::get_storable_obj_from_type(value));
            }

            // get <key> - Gets the value of the entry with the given key
            luna_action::Action::Get { key } => match map.retrieve(key.as_str()) {
                Some(value) => println!("{}", value.stringify()),
                None => println!("No value found for key {}", key),
            },

            // list - Lists all entries in the map
            luna_action::Action::Remove { key } => {
                map.remove(key.as_str());
            }

            // list - Lists all entries in the map
            luna_action::Action::List => {
                for entry in map.retrieve_all() {
                    println!("{}: {}", entry.key(), entry.value().stringify());
                }
            }

            // persist <path> - Persists the map to a file
            luna_action::Action::Clear => {
                map = memory_map::MemoryMap::new(
                    "__init".to_string(),
                    Box::new(storable::StorableInteger::new(0)),
                );
            }

            // exit - Exits the program
            luna_action::Action::Exit => {
                persist::persist_to_file(&map, &persist::get_db_file_path());
                break;
            }

            // help - Prints a list of available commands
            luna_action::Action::Help => {
                println!("Available commands:");
                println!("add <key> <value> - Adds a new entry to the map");
                println!("get <key> - Gets the value of the entry with the given key");
                println!("remove <key> - Removes the entry with the given key");
                println!("list - Lists all entries in the map");
                println!("clear - Clears the map");
                println!("exit - Exits the program");
                println!("help - Print this menu");
            }
        }
        print!("> ");
        std::io::stdout().flush().unwrap();
    }
}
