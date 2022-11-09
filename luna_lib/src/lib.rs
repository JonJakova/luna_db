use std::rc::Rc;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


struct MemoryMap<T: Storable> {
    entry: Rc<SimpleEnty<T>>,
    next: Option<Box<MemoryMap<T>>>
}

impl<T: Storable> MemoryMap<T> {
    fn new(init_key: String, init_value: T) -> MemoryMap<T> {
        MemoryMap {
            entry: Rc::new(SimpleEntry::new(init_key, init_value)),
            next: None
        }
    }
}

trait Storable {
    fn strigify(&self) -> String;
}

struct SimpleEnty<T: Storable >  {
    pub key: String,
    pub value: T,
}

impl<T: Storable> SimpleEnty<T> {
    pub fn new(key: String, value: T) -> SimpleEnty<T> {
        SimpleEnty {
            key,
            value,
        }
    }
}
