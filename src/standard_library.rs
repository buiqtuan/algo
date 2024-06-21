use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

fn string_literal() {
    let s: &str = "Hello, world!";
}

fn vec_example() {
    let mut v1 = Vec::from([1, 2, 3]);

    let closure_1 = |x: i32, y: i32| -> i32 {
        let z = x + y; z * 2;
        z
    };

    let mut v2 = vec![1, 2, 3];
}

struct Counter<T> {
    values: HashMap<T, u64>
}

impl<T: Eq + Hash> Counter<T> {
    fn new() -> Self {
        Counter {
            values: HashMap::new()
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }
    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

struct Citation {
    author: String,
    year: u32
}

impl PartialEq for Citation {
    fn eq(&self, other: &Self) -> bool {
        self.author == other.author && self.year == other.year
    }
}

impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_order => author_order
        }
    }
}