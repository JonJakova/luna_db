use std::{fs, io::Write};
use std::io::prelude::*;

use crate::storage::memory_map;
use super::file;

pub fn persist_to_file(map: &memory_map::MemoryMap, path: &str) {
    let mut file = file::create_file_if_not_exists(path);
    let mut file_content = String::new();
    for entry in map.retrieve_all() {
        file_content.push_str(&format!("{}: {}\n", entry.key, entry.value.stringify()));
    }
    file.write_all(file_content.as_bytes()).unwrap();
}

// pub fn load_from_file(path: &str) -> String {
//     let file = file::create_file_if_not_exists(path);
//     let mut file_content = String::new();
//     match file.read_to_string(&mut file_content) {
//         Ok(content) => {

//         }
//         Err(e) => {

//         }
//     }
//     return file_content;
// }