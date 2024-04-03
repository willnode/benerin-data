use std::collections::HashSet;

pub mod sastrawi;

use include_bytes_zstd::include_bytes_zstd;

pub fn get_root_words_in_hash_set() -> HashSet<String> {
    to_hash_set(include_bytes_zstd!("src/data/rootwords.txt", 2))
}

pub fn get_stop_words_in_hash_set() -> HashSet<String> {
    to_hash_set(include_bytes_zstd!("src/data/stopwords.txt", 2))
}

fn to_hash_set(data: Vec<u8>) -> HashSet<String> {
    let words_str = String::from_utf8(data).expect("error parsing");
    let word_vec = words_str.trim_end().split('\n').map(|word| word.to_string()).collect();
    return word_vec;
}
