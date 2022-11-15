use super::traits::Storable;

pub struct MemoryMap {
    entry: SimpleEntry,
    next: Option<Box<MemoryMap>>,
}

impl MemoryMap {
    pub fn new(init_key: String, init_value: Box<dyn Storable>) -> MemoryMap {
        MemoryMap {
            entry: SimpleEntry::new(init_key, init_value),
            next: None,
        }
    }

    pub fn retrieve_all(&self) -> Vec<&SimpleEntry> {
        let mut entries = Vec::new();
        let mut current = self;
        loop {
            entries.push(&current.entry);
            match &current.next {
                Some(next) => {
                    current = next;
                }
                None => {
                    break;
                }
            }
        }
        entries
    }

    pub fn retrieve(&self, key: &str) -> Option<&Box<dyn Storable>> {
        if self.entry.key == key {
            Some(&self.entry.value)
        } else {
            match &self.next {
                Some(next) => next.retrieve(key),
                None => None,
            }
        }
    }

    pub fn insert(&mut self, key: String, value: Box<dyn Storable>) {
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

    pub fn remove(&mut self, key: &str) {
        let mut current_map = self;
        let next = match &mut current_map.next {
            Some(next) => next,
            None => return,
        };
        if next.entry.key == key {
            current_map.next = next.next.take();
        } else {
            next.remove(key);
        }
    }

}

pub struct SimpleEntry {
    pub key: String,
    pub value: Box<dyn Storable>,
}

impl SimpleEntry {
    pub fn new(key: String, value: Box<dyn Storable>) -> SimpleEntry {
        SimpleEntry { key, value }
    }
}
