struct Leet {}
impl Leet {
    fn sort_vec(str: &String) -> String {
        let mut byte: Vec<_> = str.chars().collect();
        byte.sort_unstable();

        let val: String = byte.into_iter().collect();
        val
    }

    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut table: HashMap<String, Vec<String>> = HashMap::new();

        for stri in strs {
            let sorted = Self::sort_vec(&stri);

            let val_vec = table.entry(sorted).or_insert_with(|| Vec::new());
            val_vec.push(stri);
        }

        let val: Vec<Vec<String>> = table.into_values().collect();
        val
    }
}

pub fn run() {
    let inputs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let output = vec![
        vec!["bat".to_string()],
        vec!["nat".to_string(), "tan".to_string()],
        vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
    ];

    assert_eq!(output, Leet::group_anagrams(inputs));
}
