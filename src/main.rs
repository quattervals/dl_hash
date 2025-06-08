use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Default)]
struct MapItem {
    key: String,
    value: i32,
    index_prev: Option<usize>,
    index_next: Option<usize>,
}

impl MapItem {
    fn new(key: &str, value: i32) -> Self {
        MapItem {
            key: key.to_string(),
            value,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
struct HashMap {
    items: Vec<Option<MapItem>>,
    capacity: usize,
    last_touched: Option<usize>,
    first_touched: Option<usize>,
}

impl HashMap {
    fn new(size: usize) -> Self {
        let vec_size = optimal_initial_size_factor(size);
        let mut vec: Vec<Option<MapItem>> = vec![None; vec_size];
        HashMap {
            items: vec,
            capacity: vec_size,
            ..Default::default()
        }
    }

    fn update_neighbours(&mut self, index: usize) {
        if self.first_touched.is_none() {
            self.first_touched = Some(index);

            self.items[index].as_mut().unwrap().index_prev = None;
        } else {
            self.items[index].as_mut().unwrap().index_prev = self.last_touched;
            self.items[self.last_touched.unwrap()]
                .as_mut()
                .unwrap()
                .index_next = Some(index);
        }

        self.items[index].as_mut().unwrap().index_next = None;
        self.last_touched = Some(index);
    }

    /// Not handling that this function may be fallible. E.g.
    /// - there may not be enough room in the vector
    /// --> just unwrap get(index)
    fn insert(&mut self, key_val: MapItem) {
        let start_index = key_to_index(&key_val.key, self.capacity);

        let mut index = start_index;

        loop {
            match &self.items[index] {
                Some(item) => {
                    if item.key == key_val.key {
                        self.items[index] = Some(key_val);
                        break;
                    } else {
                        index = (index + 1) % self.capacity;
                        if index == start_index {
                            return;
                        }
                    }
                }

                None => {
                    self.items[index] = Some(key_val);
                    break;
                }
            }
        }

        self.update_neighbours(index);
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

    /// Rewire connections to predecessors and successors
    /// Unwrapping liberally. This is not well tested and is likely to fail with small collections
    /// But I need to stop somewhere
    fn rewire_neighbours_and_remove(&mut self, index: usize) {
        // if it is the only item
        if self.first_touched.unwrap() == self.last_touched.unwrap() {
            self.first_touched = None;
            self.last_touched = None;
        }
        // it is the first touched item
        else if self.first_touched.unwrap() == index {
            let successor_idx = self.items[index].as_ref().unwrap().index_next.unwrap();
            self.items[successor_idx].as_mut().unwrap().index_prev = None;
            self.first_touched = Some(successor_idx);
        }
        // if it is the last touched item
        else if self.last_touched.unwrap() == index {
            let pred_idx = self.items[index].as_ref().unwrap().index_prev.unwrap();
            self.items[pred_idx].as_mut().unwrap().index_next = None;
            self.last_touched = Some(pred_idx);
        }
        // if the item is in between
        else {
            let successor_idx = self.items[index].as_ref().unwrap().index_next.unwrap();
            let pred_idx = self.items[index].as_ref().unwrap().index_prev.unwrap();

            self.items[pred_idx].as_mut().unwrap().index_next = Some(successor_idx);
            self.items[successor_idx].as_mut().unwrap().index_prev = Some(pred_idx);
        }

        // let current_item = self.items[index].as_ref().unwrap();

        // println!(
        //     "previous {:#?}",
        //     self.items[current_item.index_prev.unwrap()]
        //         .as_ref()
        //         .unwrap()
        // );
        // println!(" current {:#?}", self.items[index].as_mut().unwrap());
        // println!("index {index}");

        self.items[index] = None;
    }

    fn remove(&mut self, key: &str) {
        let start_index = key_to_index(key, self.capacity);
        let mut index = start_index;

        loop {
            match &self.items[index] {
                Some(item) if item.key == key => self.rewire_neighbours_and_remove(index),
                Some(_) => {
                    index = (index + 1) % self.capacity;
                    if index == start_index {
                        return;
                    }
                }
                None => return,
            }
        }
    }

    fn get_first(&self) -> Option<MapItem> {
        if self.first_touched.is_some() {
            self.items[self.first_touched.unwrap()].clone()
        } else {
            None
        }
    }

    fn get_last(&self) -> Option<MapItem> {
        match self.last_touched {
            Some(idx) => Some(self.items[idx].as_ref().unwrap().clone()),
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

    fn items_equal(left: &MapItem, right: &MapItem) {
        assert_eq!(left.key, right.key);
        assert_eq!(left.value, right.value);
    }

    #[test]
    fn insert_key() {
        let mut hmap = HashMap::new(TEST_WORDS.len());

        let item = MapItem::new("test", 33);
        hmap.insert(item);

        let inserted_value = hmap.get("test");
        assert_eq!(33, inserted_value.unwrap());
    }

    #[test]
    fn update_value() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        hmap.insert(MapItem::new(TEST_WORDS[2], 77));

        let updated_value = hmap.get(TEST_WORDS[2]);
        assert_eq!(77, updated_value.unwrap());
    }

    #[test]
    fn insert_colliding_key() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        let item = MapItem::new(COLLIDING_WORDS[0], 33);
        hmap.insert(item);

        let inserted_value = hmap.get(COLLIDING_WORDS[0]);
        assert_eq!(33, inserted_value.unwrap());
    }

    #[test]
    fn remove_key() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        hmap.remove(TEST_WORDS[3]);

        let deleted_value = hmap.get(TEST_WORDS[3]);
        assert_eq!(None, deleted_value);
    }

    #[test]
    fn get_first_pair() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        let first_item = hmap.get_first();

        items_equal(&MapItem::new(TEST_WORDS[0], 100), &first_item.unwrap());
    }

    #[test]
    fn get_first_after_deletion_of_veryfirst() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        hmap.remove(TEST_WORDS[0]);

        let first_item = hmap.get_first();
        items_equal(&MapItem::new(TEST_WORDS[1], 101), &first_item.unwrap());
    }

    #[test]
    fn get_last_after_deletion_of_last() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        hmap.remove(TEST_WORDS[3]);

        let last_item = hmap.get_last();
        items_equal(&MapItem::new(TEST_WORDS[2], 102), &last_item.unwrap());
    }

    #[test]
    fn get_last_item_after_central_deletion() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        hmap.remove(TEST_WORDS[2]);
        hmap.remove(TEST_WORDS[3]);

        let last_item = hmap.get_last();
        items_equal(&MapItem::new(TEST_WORDS[1], 101), &last_item.unwrap());
    }

    #[test]
    fn get_last_touched_pair() {
        let mut hmap = HashMap::new(TEST_WORDS.len());
        for word in TEST_WORDS.iter().enumerate() {
            hmap.insert(MapItem::new(word.1, word.0 as i32 + 100));
        }

        let last_item = hmap.get_last();

        items_equal(&MapItem::new(TEST_WORDS[3], 103), &last_item.unwrap());
    }
}
