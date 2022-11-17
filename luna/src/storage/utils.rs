use super::storable;

pub fn get_storable_type(value: &str) -> storable::StorableType {
    match value {
        "String" => storable::StorableType::STRING,
        "Integer" => storable::StorableType::INTEGER,
        "Float" => storable::StorableType::FLOAT,
        "Boolean" => storable::StorableType::BOOLEAN,
        "Array" => storable::StorableType::ARRAY,
        "Object" => storable::StorableType::OBJECT,
        "Null" => storable::StorableType::NULL,
        _ => panic!("Unknown type: {}", value),
    }
}
