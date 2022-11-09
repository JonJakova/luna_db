use std::collections::HashMap;
use super::traits::Storable;

pub struct StorableString {
    pub value: String,
}

impl Storable for StorableString {
    fn stringify(&self) -> String {
        self.value.clone()
    }
}

impl StorableString {
    pub fn new(value: String) -> StorableString {
        StorableString {
            value,
        }
    }
}

pub struct StorableInteger {
    pub value: i32,
}

impl Storable for StorableInteger {
    fn stringify(&self) -> String {
        self.value.to_string()
    }
}

impl StorableInteger {
    pub fn new(value: i32) -> StorableInteger {
        StorableInteger {
            value,
        }
    }
}

pub struct StorableFloat {
    pub value: f32,
}

impl Storable for StorableFloat {
    fn stringify(&self) -> String {
        self.value.to_string()
    }
}

impl StorableFloat {
    pub fn new(value: f32) -> StorableFloat {
        StorableFloat {
            value,
        }
    }
}

pub struct StorableBoolean {
    pub value: bool,
}

impl Storable for StorableBoolean {
    fn stringify(&self) -> String {
        self.value.to_string()
    }
}

impl StorableBoolean {
    pub fn new(value: bool) -> StorableBoolean {
        StorableBoolean {
            value,
        }
    }
}

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
}

impl StorableArray {
    pub fn new(value: Vec<Box<dyn Storable>>) -> StorableArray {
        StorableArray {
            value,
        }
    }
}

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
}

impl StorableObject {
    pub fn new(value: HashMap<String, Box<dyn Storable>>) -> StorableObject {
        StorableObject {
            value,
        }
    }
}

pub struct StorableNull;

impl Storable for StorableNull {
    fn stringify(&self) -> String {
        String::from("null")
    }
}

impl StorableNull {
    pub fn new() -> StorableNull {
        StorableNull {}
    }
}

pub struct StorableUndefined;

impl Storable for StorableUndefined {
    fn stringify(&self) -> String {
        String::from("undefined")
    }
}

impl StorableUndefined {
    pub fn new() -> StorableUndefined {
        StorableUndefined {}
    }
}
