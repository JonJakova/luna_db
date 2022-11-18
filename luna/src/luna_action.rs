use std::io;

pub enum Action {
    Add { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    List,
    Clear,
    Exit,
    Help,
}

pub fn get_action() -> Action {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut input = input.split_whitespace();
    let action = input.next().unwrap();
    match action {
        "add" => {
            let key = input.next().unwrap().to_string();
            let value = input.next().unwrap().to_string();
            Action::Add { key, value }
        }
        "get" => {
            let key = input.next().unwrap().to_string();
            Action::Get { key }
        }
        "remove" | "rm" => {
            let key = input.next().unwrap().to_string();
            Action::Remove { key }
        }
        "list" | "ls" => Action::List,
        "clear" => Action::Clear,
        "exit" => Action::Exit,
        "help" => Action::Help,
        _ => {
            println!("Invalid action");
            get_action()
        }
    }
}
