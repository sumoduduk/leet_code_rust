pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::collections::HashMap;
    let words: Vec<_> = s.split_whitespace().collect();

    if words.len() != pattern.len() {
        return false;
    }

    let mut map = HashMap::new();

    let mut legit = false;

    for (i, char) in pattern.char_indices() {
        let map_value = map.insert(char, words[i]);

        match map_value {
            Some(m) => {
                if m != words[i] {
                    legit = false;
                }
            }
            None => {
                legit = true;
            }
        }
    }

    let mut sort_dedup = words.clone();
    sort_dedup.sort();
    sort_dedup.dedup();

    if map.len() != sort_dedup.len() {
        legit = false;
    }

    legit
}

pub fn run() {
    let pattern = "abba".to_string();
    let s = "dog cat cat bird".to_owned();

    // let pattern = "abba".to_string();
    // let s = "dog dog dog dog".to_owned();

    let val = word_pattern(pattern, s);

    assert_eq!(false, val);
}
