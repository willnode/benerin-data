use std::collections::{HashMap, HashSet};

pub mod sastrawi;

use include_bytes_zstd::include_bytes_zstd;

pub fn get_root_words_in_hash_set() -> HashSet<String> {
    to_hash_set(include_bytes_zstd!("src/data/rootwords.txt", 2))
}

pub fn get_prefiks_indexed_in_hash_map() -> HashMap<String, (bool, Vec<String>)> {
    let mut map: HashMap<String, (bool, Vec<String>)> = HashMap::new();
    for r in get_prefiks_in_hash_set().into_iter() {
        if r.contains('-') {
            // pseudo
            let splits: Vec<&str> = r.split('-').collect();
            match map.get_mut(splits[0]) {
                Some((_, leks)) => leks.push(splits[1].to_owned()),
                None => {
                    let n = splits[1].to_owned();
                    map.insert(splits[0].to_owned(), (true, vec![n]));
                }
            }
            continue;
        }
        for j in 1..r.len() {
            let p = &r[0..j];
            if !map.contains_key(p) {
                map.insert(p.to_owned(), (false, vec![]));
            }
        }
        match map.get_mut(&r) {
            Some((v, _)) => {
                *v = true;
            }
            None => {
                map.insert(r, (true, vec![]));
            }
        }
    }
    map
}

pub fn get_suffiks_indexed_in_hash_map() -> HashMap<String, bool> {
    let mut map: HashMap<String, bool> = HashMap::new();
    for r in get_suffiks_in_hash_set().into_iter() {
        for j in 1..r.len() {
            let p = &r[r.len() - j..r.len()];
            if !map.contains_key(p) {
                map.insert(p.to_owned(), false);
            }
        }
        map.insert(r, true);
    }
    map
}

pub fn get_prefiks_in_hash_set() -> HashSet<String> {
    to_hash_set(include_bytes_zstd!("src/data/prefiks.txt", 2))
}

pub fn get_suffiks_in_hash_set() -> HashSet<String> {
    to_hash_set(include_bytes_zstd!("src/data/suffiks.txt", 2))
}

pub fn get_stop_words_in_hash_set() -> HashSet<String> {
    to_hash_set(include_bytes_zstd!("src/data/stopwords.txt", 2))
}

fn to_hash_set(data: Vec<u8>) -> HashSet<String> {
    let words_str = String::from_utf8(data).expect("error parsing");
    let word_vec = words_str
        .trim_end()
        .split('\n')
        .map(|word| word.to_string())
        .collect();
    return word_vec;
}
