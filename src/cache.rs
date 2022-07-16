use std::collections::HashMap;

pub fn get<'a>(store: HashMap<&'a str, &'a str>, key: &'a str) -> &'a str {
    match store.get(&key) {
        Some(&v) => v,
        None => "",
    }
}

pub fn set<'a>(store: &mut HashMap<&'a str, &'a str>, key: &'a str, value: &'a str) {
    store.insert(key, value);
}