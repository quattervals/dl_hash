use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
struct MapItem {
    key: String,
    value: i32,
}

#[derive(Debug)]
struct HashMap {
    items: Vec<Option<MapItem>>,
    capacity: usize,
    // last_item: Option<MapItem>,
    // first_item: Option<MapItem>,
}

impl HashMap {
    fn new(size: usize) -> Self {
        let vec_size = optimal_initial_size_factor(size);
        let mut vec: Vec<Option<MapItem>> = vec![None; vec_size];
        HashMap {
            items: vec,
            capacity: vec_size,
        }
    }

    /// Not handling that this function may be fallible. E.g.
    /// - there may not be enough room in the vector
    /// --> just unwrap get(index)
    fn insert(&mut self, key_val: MapItem) {
        let start_index = key_to_index(&key_val.key, self.capacity);

        let mut index = start_index;

        while self.items.get(index).unwrap().is_some() {
            index = (index + 1) % self.capacity;
            if index == start_index {
                return;
            }
        }

        self.items[index] = Some(key_val);
    }

    fn get(&self, key: &str) -> Option<i32> {
        let start_index = key_to_index(key, self.capacity);
        let mut index = start_index;

        loop {
            match &self.items[index] {
                Some(item) if item.key == key => return Some(item.value),
                Some(_) => {
                    index = (index + 1) % self.capacity;
                    if index == start_index {
                        return None;
                    }
                }
                None => return None,
            }
        }
    }

    fn remove(&mut self, key: &str) {
        let start_index = key_to_index(key, self.capacity);
        let mut index = start_index;

        loop {
            match &self.items[index] {
                Some(item) if item.key == key => self.items[index] = None,
                Some(_) => {
                    index = (index + 1) % self.capacity;
                    if index == start_index {
                        break;
                    }
                }
                None => break,
            }
        }
    }
}

/// We haven't had a discussion about performance trade-offs
/// One point is to make the vector as short as possible
/// Another point is to reduce the amount of collisions -> better with long vector
/// So this function only returns a heuristic tuning value
fn optimal_initial_size_factor(initial_guess: usize) -> usize {
    initial_guess * 2
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
fn key_to_index(key: &str, len: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % len
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let book_raw = fs::read_to_string("cd.txt")?;

    // This regex finds words between word boundaries
    // i.e. anything matching word characters between non-word characters
    let re = Regex::new(r"\b\w+\b").unwrap();
    let words: Vec<&str> = re.find_iter(&book_raw).map(|m| m.as_str()).collect();

    for word in &words {
        println!("word {word}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WORDS: [&str; 4] = ["abcd", "1984", "Gutenberg", "eBook"];
    const COLLIDING_WORDS: [&str; 1] = ["asdf"];

    #[test]
    fn insert_key() {
        let mut hmap = HashMap::new(TEST_WORDS.len());

        let item = MapItem {
            key: "test".to_string(),
            value: 33,
        };
        hmap.insert(item);

        let inserted_value = hmap.get("test");
        assert_eq!(33, inserted_value.unwrap());
    }

    #[test]
    fn insert_colliding_key() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem {
                key: word.1.to_string(),
                value: word.0 as i32 + 100,
            });
        }

        let item = MapItem {
            key: COLLIDING_WORDS[0].to_string(),
            value: 33,
        };
        hmap.insert(item);

        let inserted_value = hmap.get(COLLIDING_WORDS[0]);
        assert_eq!(33, inserted_value.unwrap());
    }

    #[test]
    fn remove_key() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem {
                key: word.1.to_string(),
                value: word.0 as i32 + 100,
            });
        }

        hmap.remove(TEST_WORDS[3]);

        let deleted_value = hmap.get(TEST_WORDS[3]);
        assert_eq!(None, deleted_value);
    }
}
