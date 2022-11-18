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

pub fn get_storable_type_from_value(value: &str) -> storable::StorableType {
    if let Ok(_) = value.parse::<i32>() {
        return storable::StorableType::INTEGER;
    }
    if let Ok(_) = value.parse::<f64>() {
        return storable::StorableType::FLOAT;
    }
    if let Ok(_) = value.parse::<bool>() {
        return storable::StorableType::BOOLEAN;
    }
    if value == "null" {
        return storable::StorableType::NULL;
    }
    storable::StorableType::STRING
}

pub fn get_storable_obj_from_type(value: String) -> Box<dyn storable::Storable> {
    let storable_type = get_storable_type_from_value(&value);
    match storable_type {
        storable::StorableType::STRING => Box::new(storable::StorableString::new(value)),
        storable::StorableType::INTEGER => Box::new(storable::StorableInteger::new(value.parse::<i32>().unwrap())),
        storable::StorableType::FLOAT => Box::new(storable::StorableFloat::new(value.parse::<f64>().unwrap())),
        storable::StorableType::BOOLEAN => Box::new(storable::StorableBoolean::new(value.parse::<bool>().unwrap())),
        storable::StorableType::NULL => Box::new(storable::StorableNull::new()),
        _ => panic!("Unknown type: {}", value),
    }
}