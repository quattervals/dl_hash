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
        HashMap { items: vec,
        capacity: vec_size }
    }

    /// Not handling that this function may be fallible. E.g.
    /// - there may not be enough room in the vector
    /// - not handling wrap-around at the end of the vector
    fn insert(&mut self, key_val: MapItem) {
        // calculate hash of String

        // check index, move on if not present

        let index = key_to_index(&key_val.key, self.capacity);
        self.items.insert(index, Some(key_val));
    }

    fn get(&self, key: &str) -> Option<i32> {
        let index = key_to_index(key, self.capacity);
        let thing_at_index: Option<&Option<MapItem>> = self.items.get(index);

        match thing_at_index {
            Some(i) => match i {
                Some(ii) => Some(ii.value),
                None => None,
            },
            None => None,
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


use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
fn key_to_index(key: &str, len: usize) -> usize {


    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize ) % len
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
    #[test]
    fn insert_key() {
        let mut hmap = HashMap::new(TEST_WORDS.len());

        let item = MapItem {
            key: "test".to_string(),
            value: 33,
        };
        hmap.insert(item);

        println!("map {:#?}", hmap);

        let inserted_value = hmap.get("test");
        assert_eq!(33, inserted_value.unwrap());
    }

    
    // same word twice
    // index collision
}
