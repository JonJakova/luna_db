use std::io::{Read, Write};

use super::file;
use crate::storable;
use crate::storage::utils;
use crate::storage::memory_map;

pub fn persist_to_file(map: &memory_map::MemoryMap, path: &str) {
    let mut file = file::create_file_if_not_exists(path);
    let mut file_content = String::new();
    for entry in map.retrieve_all() {
        file_content.push_str(&format!(
            "{}: {} #{}\n",
            entry.key(),
            entry.value().stringify(),
            entry.value().as_type().as_str()
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
        let entry_type = utils::get_storable_type(entry_type);
        match entry_type {
            storable::StorableType::STRING => {
                map.insert(
                    key.to_string(),
                    Box::new(storable::StorableString::new(value.to_string())),
                );
            }
            storable::StorableType::INTEGER => {
                map.insert(
                    key.to_string(),
                    Box::new(storable::StorableInteger::new(
                        value.parse::<i32>().unwrap(),
                    )),
                );
            }
            storable::StorableType::FLOAT => {
                map.insert(
                    key.to_string(),
                    Box::new(storable::StorableFloat::new(value.parse::<f64>().unwrap())),
                );
            }
            storable::StorableType::BOOLEAN => {
                map.insert(
                    key.to_string(),
                    Box::new(storable::StorableBoolean::new(value.parse::<bool>().unwrap())),
                );
            }
            // storable::StorableType::ARRAY => {
            //     map.insert(
            //         key.to_string(),
            //         Box::new(storable::StorableArray::new(value.to_string())),
            //     );
            // }
            // storable::StorableType::OBJECT => {
            //     map.insert(
            //         key.to_string(),
            //         Box::new(storable::StorableObject::new(value.to_string())),
            //     );
            // }
            _ => {
                panic!("Unknown type: {}", entry_type.as_str());
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
