mod storage;

use storage::memory_map;
use storage::storable;

fn main() {
    let mut map = memory_map::MemoryMap::new(
        "key_str".to_string(),
        Box::new(storable::StorableString::new("str_value".to_string())),
    );

    map.insert("key_bool".to_string(), Box::new(storable::StorableBoolean::new(true)));

    map.insert("key_arr".to_string(), Box::new(storable::StorableArray::new(vec![
        Box::new(storable::StorableString::new("str_value".to_string())),
        Box::new(storable::StorableBoolean::new(true)),
    ])));

    // display all entries
    for entry in map.retrieve_all() {
        println!("{}: {}", entry.key, entry.value.stringify());
    }

}
