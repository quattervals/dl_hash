use regex::Regex;
use std::fs;

#[derive(Debug)]
struct MapItem {
    key: String,
    value: i32,
}

#[derive(Debug)]
struct HashMap {
    items: Vec<MapItem>,
    // last_item: Option<MapItem>,
    // first_item: Option<MapItem>,
}

impl HashMap {
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size * optimal_initial_size_factor(size));
        HashMap { items: vec }
    }
}

/// We haven't had a discussion about performance trade-offs
/// One point is to make the vector as short as possible
/// Another point is to reduce the amount of collisions -> better with long vector
/// So this function only returns a heuristic tuning value
fn optimal_initial_size_factor(initial_guess: usize) -> usize {
    initial_guess * 2
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
    fn create_hashmap() {
        let hmap = HashMap::new(TEST_WORDS.len());
        println!("map {:#?}", hmap);
    }
}
