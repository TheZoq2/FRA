use std::collections::HashSet;
use std::path::Path;

use std::fs::File;
use std::io::prelude::*;

fn load_dictionary(file: &Path) -> HashSet<String> {
    let mut file = File::open(file)
        .expect(&format!("failed to load dictionary {:?}", file));
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read dictionary content");
    content.lines().map(Into::into).collect()
}


fn pairs_from_string(input: &str) -> Vec<(&str, &str)> {
    input.split_ascii_whitespace()
        .zip(input.split_ascii_whitespace().skip(1))
        .collect()
}

fn should_be_flagged<'a>(
    words: &[(&'a str, &'a str)],
    dictionary: HashSet<String>
) -> Vec<(&'a str, &'a str)> {
    words.iter()
        .filter(|(a, b)| dictionary.contains(&format!("{}{}", a, b)))
        .cloned()
        .collect()
}

fn main() {
    let dictionary = load_dictionary(Path::new("wordlist.txt"));

    let result = pairs_from_string("Edvard gillar att sär skriva");

    println!("{:?}", should_be_flagged(&result, dictionary));
}
