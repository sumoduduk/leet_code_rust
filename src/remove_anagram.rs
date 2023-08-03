fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let mut val = Vec::with_capacity(words.len());

    let mut key = [0; 26];

    for w in words {
        let cas = cast(&w);
        if cas != key {
            val.push(w)
        }
        key = cas;
    }

    val
}

fn cast(st: &str) -> [usize; 26] {
    let mut cas = [0; 26];

    for s in st.bytes() {
        cas[(s - b'a') as usize] += 1;
    }
    cas
}

pub fn run() {
    let inputs = vec![
        "abba".to_string(),
        "baba".to_string(),
        "bbaa".to_string(),
        "cd".to_string(),
        "cd".to_string(),
    ];
    let output = vec!["abba".to_string(), "cd".to_string()];

    assert_eq!(output, remove_anagrams(inputs));
}
