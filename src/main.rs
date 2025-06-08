use regex::Regex;
use std::fs;

struct MapItem {
    key: String,
    value: i32,
}

struct HashMap(Vec<MapItem>);

impl HashMap{

    fn new(size:usize) -> Self{

        let mut vec = Vec::with_capacity(size);
        HashMap(vec)

    }

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
    fn create_hashmap()
    {
        let hmap = HashMap::new(TEST_WORDS.len());
    }

}
