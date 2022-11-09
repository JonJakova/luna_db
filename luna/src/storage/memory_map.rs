use super::traits::Storable;

pub struct MemoryMap<T: Storable> {
    entry: Box<SimpleEntry<T>>,
    next: Option<Box<MemoryMap<T>>>,
}

impl<T: Storable> MemoryMap<T> {
    pub fn new(init_key: String, init_value: T) -> MemoryMap<T> {
        MemoryMap {
            entry: Box::new(SimpleEntry::new(init_key, init_value)),
            next: None,
        }
    }

    pub fn retrieve(&self, key: String) -> Option<&T> {
        if self.entry.key == key {
            Some(&self.entry.value)
        } else {
            match &self.next {
                Some(next) => next.retrieve(key),
                None => None,
            }
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        let mut current_map = self;
        loop {
            if current_map.entry.key == key {
                current_map.entry.value = value;
                return;
            }
            match current_map.next {
                Some(ref mut next) => current_map = next,
                None => {
                    current_map.next = Some(Box::new(MemoryMap::new(key, value)));
                    return;
                }
            }
        }
    }
}

pub struct SimpleEntry<T: Storable> {
    pub key: String,
    pub value: T,
}

impl<T: Storable> SimpleEntry<T> {
    pub fn new(key: String, value: T) -> SimpleEntry<T> {
        SimpleEntry { key, value }
    }
}
