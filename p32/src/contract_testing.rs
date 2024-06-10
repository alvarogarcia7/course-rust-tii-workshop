use std::collections::HashMap;
use std::hash::Hash;

trait AccessToData<K, T>: Default {
    fn insert(&mut self, key: K, item: T);
    fn remove(&mut self, key: &K);
}

struct MyHM<K: Eq + Hash, T> {
    value: HashMap<K, T>,
}

impl<K: Eq + Hash, T> Default for MyHM<K, T> {
    fn default() -> Self {
        MyHM {
            value: HashMap::<K, T>::default(),
        }
    }
}

impl<K: Eq + Hash, T> AccessToData<K, T> for MyHM<K, T> {
    fn insert(&mut self, key: K, item: T) {
        self.value.insert(key, item);
    }

    fn remove(&mut self, key: &K) {
        self.value.remove(key);
    }
}

pub mod tests {
    use super::*;

    fn generic_test<T: AccessToData<u32, u64>>() {
        let mut c = T::default();
        c.insert(1, 2);
        c.remove(&1);
    }

    #[test]
    fn test_hash_map() {
        // let mut h = HashMap::<u32, u64>::new();
        generic_test::<MyHM<u32, u64>>();
    }
}
