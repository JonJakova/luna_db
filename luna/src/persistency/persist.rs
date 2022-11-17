use std::io::{Read, Write};

use super::file;
use crate::storable;
use crate::storage::memory_map;

pub fn persist_to_file(map: &memory_map::MemoryMap, path: &str) {
    let mut file = file::create_file_if_not_exists(path);
    let mut file_content = String::new();
    for entry in map.retrieve_all() {
        file_content.push_str(&format!(
            "{}: {} #{}\n",
            entry.key(),
            entry.value().stringify(),
            entry.value().as_type()
        ));
    }
    file.write_all(file_content.as_bytes()).unwrap();
}

pub fn load_from_file(path: &str) -> memory_map::MemoryMap {
    let content = load_from_file_as_string(path);
    let mut map: memory_map::MemoryMap = memory_map::MemoryMap::new(
        "__init__".to_string(),
        Box::new(storable::StorableInteger::new(0)),
    );
    let mut lines = content.lines();
    lines.next();

    for line in content.lines() {
        let mut parts = line.split(" #");
        let entry = parts.next().unwrap();
        let entry_type = parts.next().unwrap();
        let mut entry_parts = entry.split(": ");
        let key = entry_parts.next().unwrap();
        let value = entry_parts.next().unwrap();
        match entry_type {
            "String" => {
                map.insert(
                    key.to_string(),
                    Box::new(storable::StorableString::new(value.to_string())),
                );
            }
            "Integer" => {
                map.insert(
                    key.to_string(),
                    Box::new(storable::StorableInteger::new(
                        value.parse::<i32>().unwrap(),
                    )),
                );
            }
            "Float" => {
                map.insert(
                    key.to_string(),
                    Box::new(storable::StorableFloat::new(value.parse::<f64>().unwrap())),
                );
            }
            _ => {
                panic!("Unknown type: {}", entry_type);
            }
        }
    }

    return map;
}

fn load_from_file_as_string(path: &str) -> String {
    let mut file_content = String::new();
    if !file::does_file_exists(path) {
        return file_content;
    }
    let mut file = file::create_file_if_not_exists(path);
    file.read_to_string(&mut file_content).unwrap();
    return file_content;
}
