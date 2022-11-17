use super::traits::Storable;
use std::collections::HashMap;

// String
pub struct StorableString {
    value: String,
}

impl Storable for StorableString {
    fn stringify(&self) -> String {
        self.value.clone()
    }
    fn as_type(&self) -> String {
        "String".to_string()
    }
}

impl StorableString {
    pub fn new(value: String) -> StorableString {
        StorableString { value }
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

// Interger
pub struct StorableInteger {
    value: i32,
}

impl Storable for StorableInteger {
    fn stringify(&self) -> String {
        self.value.to_string()
    }

    fn as_type(&self) -> String {
        "Integer".to_string()
    }
}

impl StorableInteger {
    pub fn new(value: i32) -> StorableInteger {
        StorableInteger { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn from_str(value: &str) -> StorableInteger {
        StorableInteger {
            value: value.parse::<i32>().unwrap(),
        }
    }

    pub fn is_struct(&self, val: &str) -> bool {
        val.parse::<i32>().is_ok()
    }
}

// Float
pub struct StorableFloat {
    value: f64,
}

impl Storable for StorableFloat {
    fn stringify(&self) -> String {
        self.value.to_string()
    }

    fn as_type(&self) -> String {
        "Float".to_string()
    }
}

impl StorableFloat {
    pub fn new(value: f64) -> StorableFloat {
        StorableFloat { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn from_str(value: &str) -> StorableFloat {
        StorableFloat {
            value: value.parse::<f64>().unwrap(),
        }
    }

    pub fn is_struct(&self, val: &str) -> bool {
        val.parse::<f64>().is_ok()
    }
}

// Boolean
pub struct StorableBoolean {
    value: bool,
}

impl Storable for StorableBoolean {
    fn stringify(&self) -> String {
        self.value.to_string()
    }

    fn as_type(&self) -> String {
        "Boolean".to_string()
    }
}

impl StorableBoolean {
    pub fn new(value: bool) -> StorableBoolean {
        StorableBoolean { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn from_str(value: &str) -> StorableBoolean {
        StorableBoolean {
            value: value.parse::<bool>().unwrap(),
        }
    }

    pub fn is_struct(&self, val: &str) -> bool {
        val.parse::<bool>().is_ok()
    }
}

// Vector
pub struct StorableArray {
    pub value: Vec<Box<dyn Storable>>,
}

impl Storable for StorableArray {
    fn stringify(&self) -> String {
        let mut result = String::from("[");
        for item in &self.value {
            result.push_str(&item.stringify());
            result.push_str(", ");
        }
        result.push_str("]");
        result
    }

    fn as_type(&self) -> String {
        "Array".to_string()
    }
}

impl StorableArray {
    pub fn new(value: Vec<Box<dyn Storable>>) -> StorableArray {
        StorableArray { value }
    }

    pub fn value(&self) -> &Vec<Box<dyn Storable>> {
        &self.value
    }

    // Currently supports only one-dimensional String arrays
    pub fn from_str(value: &str) -> StorableArray {
        let mut result: Vec<Box<dyn Storable>> = Vec::new();
        let mut current = String::new();
        let mut in_string = false;

        for c in value.chars() {
            if c == '"' {
                in_string = !in_string;
            }
            if c == ',' && !in_string {
                result.push(Box::new(StorableString::new(current.clone())));
                current = String::new();
            } else {
                current.push(c);
            }
        }

        result.push(Box::new(StorableString::new(current.clone())));
        return StorableArray::new(result);
    }

    pub fn is_struct(&self, val: &str) -> bool {
        val.starts_with("[") && val.ends_with("]")
    }
}

// HashMap
pub struct StorableObject {
    pub value: HashMap<String, Box<dyn Storable>>,
}

impl Storable for StorableObject {
    fn stringify(&self) -> String {
        let mut result = String::from("{");
        for (key, value) in &self.value {
            result.push_str(&key);
            result.push_str(": ");
            result.push_str(&value.stringify());
            result.push_str(", ");
        }
        result.push_str("}");
        result
    }

    fn as_type(&self) -> String {
        "Object".to_string()
    }
}

impl StorableObject {
    pub fn new(value: HashMap<String, Box<dyn Storable>>) -> StorableObject {
        StorableObject { value }
    }

    pub fn value(&self) -> &HashMap<String, Box<dyn Storable>> {
        &self.value
    }

    pub fn from_str(value: &str) -> StorableObject {
        let mut result: HashMap<String, Box<dyn Storable>> = HashMap::new();
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut in_string = false;
        let mut in_key = true;

        for c in value.chars() {
            if c == '"' {
                in_string = !in_string;
            }
            if c == ':' && !in_string {
                in_key = false;
            }
            if c == ',' && !in_string {
                result.insert(
                    current_key.clone(),
                    Box::new(StorableString::new(current_value.clone())),
                );
                current_key = String::new();
                current_value = String::new();
                in_key = true;
            } else {
                if in_key {
                    current_key.push(c);
                } else {
                    current_value.push(c);
                }
            }
        }

        result.insert(
            current_key.clone(),
            Box::new(StorableString::new(current_value.clone())),
        );
        return StorableObject::new(result);
    }
}

// Null
pub struct StorableNull;

impl Storable for StorableNull {
    fn stringify(&self) -> String {
        String::from("null")
    }

    fn as_type(&self) -> String {
        "Null".to_string()
    }
}

impl StorableNull {
    pub fn new() -> StorableNull {
        StorableNull {}
    }
}
