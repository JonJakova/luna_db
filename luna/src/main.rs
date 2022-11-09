mod storage;

use storage::memory_map;
use storage::storable;

fn main() {
    let mut map = memory_map::MemoryMap::new(
        "key_str".to_string(),
        storable::StorableString::new("str_value".to_string()),
    );

    map.insert("key_bool".to_string(), storable::StorableBoolean::new(true));
}
