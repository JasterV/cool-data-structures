use std::collections::{hash_map::DefaultHasher, LinkedList};
use std::hash::{Hash, Hasher};

pub struct HashSet<T> {
    data: Vec<Option<LinkedList<T>>>,
    size: usize,
    load_factor: f32,
    threshold: usize,
    capacity: usize,
}

impl<T: Hash + PartialEq + Clone> HashSet<T> {
    const INITIAL_CAPACITY: usize = 16;
    const MAXIMUM_CAPACITY: usize = 1 << 30;
    const DEFAULT_LOAD_FACTOR: f32 = 0.75;

    pub fn new() -> Self {
        let mut data = Vec::new();
        data.resize_with(Self::INITIAL_CAPACITY, || None);
        HashSet {
            data,
            capacity: Self::INITIAL_CAPACITY,
            load_factor: Self::DEFAULT_LOAD_FACTOR,
            threshold: (Self::INITIAL_CAPACITY as f32 * Self::DEFAULT_LOAD_FACTOR) as usize,
            size: 0,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let index = self._get_index(value);
        let link_list = &self.data[index];
        match link_list {
            Some(list) => list.contains(&value),
            None => false,
        }
    }

    pub fn add(&mut self, value: T) -> bool {
        if self.contains(&value) {
            return false;
        };
        let index = self._get_index(&value);
        match self.data[index] {
            Some(ref mut list) => list.push_front(value),
            None => {
                self.data[index] = {
                    let mut list = LinkedList::new();
                    list.push_back(value);
                    Some(list)
                }
            }
        }
        self.size += 1;
        if self.size >= self.threshold {
            self._resize();
        }
        return true;
    }

    fn _get_index(&self, value: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    fn _resize(&mut self) {
        self.capacity = self.capacity * 2;
        self.threshold = (self.capacity as f32 * self.load_factor) as usize;

        if self.capacity > Self::MAXIMUM_CAPACITY {
            panic!("Maximum capacity raised");
        }

        let data = self.data.clone();

        self.size = 0;
        self.data = Vec::new();
        self.data.resize_with(self.capacity, || None);

        data.into_iter().for_each(|list| {
            if let Some(list) = list {
                list.into_iter().for_each(|value| {
                    self.add(value);
                });
            }
        });
    }
}
