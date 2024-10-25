use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct VarSet {
    items: HashMap<TypeId, Box<dyn Any>>,
}

impl VarSet {
    pub fn new() -> VarSet {
        VarSet {
            items: HashMap::new(),
        }
    }
    pub fn insert<T: Any>(&mut self, item: T) {
        if self.items.contains_key(&TypeId::of::<T>()) {
            panic!("tried to insert an already existing value")
        }
        self.items.insert(TypeId::of::<T>(), Box::new(item));
    }
    pub fn remove<T: Any>(&mut self) -> Option<T> {
        if let Some(item) = self.items.remove(&TypeId::of::<T>()) {
            Some(*item.downcast().unwrap())
        } else {
            None
        }
    }
    pub fn get<T: Any>(&self) -> Option<&T> {
        if let Some(item) = self.items.get(&TypeId::of::<T>()) {
            Some(item.downcast_ref().unwrap())
        } else {
            None
        }
    }
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        if let Some(item) = self.items.get_mut(&TypeId::of::<T>()) {
            Some(item.downcast_mut().unwrap())
        } else {
            None
        }
    }
    pub fn contains<T: Any>(&self) -> bool {
        self.items.contains_key(&TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::VarSet;

    #[test]
    fn insert_remove() {
        struct T {
            number: u32,
            list: Vec<u8>,
        }

        let mut set = VarSet::new();

        set.insert(0u32);
        set.insert(String::from("hiii"));
        set.insert(T {
            number: 123,
            list: vec![1, 2, 3],
        });

        *set.get_mut::<u32>().unwrap() = 445;
        (*set.get_mut::<String>().unwrap()).push_str(" everyone!!");
        (*set.get_mut::<T>().unwrap()).list = vec![3,2,1];
        (*set.get_mut::<T>().unwrap()).number = 4325;
    }

    // boxed any elapsed 10: 30.721µs
    // boxed any elapsed 100: 119.404µs
    // boxed any elapsed 1_000: 1.04219ms
    // boxed any elapsed 10_000: 10.40362ms
    // boxed any elapsed 100_000: 100.257828ms
    #[test]
    fn stress() {
        let start = Instant::now();
        let mut set = VarSet::new();
        set.insert::<f32>(3241.43);
        set.insert::<u8>(123);
        set.insert::<String>(String::from("hiii"));
        for i in 0..10 {
            *set.get_mut::<f32>().unwrap() = (i % 255) as f32;
            *set.get_mut::<u8>().unwrap() = (i % 255) as u8;
            *set.get_mut::<String>().unwrap() = format!("{}", i);
        }
        println!("elapsed: {:?}", start.elapsed());
    }
}
