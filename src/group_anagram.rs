fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut byte_strs: Vec<Vec<u8>> = Vec::with_capacity(strs.len());

    for elm in strs {
        let mut byte = elm.into_bytes();
        byte.sort();
        byte_strs.push(byte);
    }

    byte_strs.sort();
    byte_strs.dedup();

    dbg!(byte_strs);

    vec![vec!["".to_string()]]
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

    assert_eq!(output, group_anagrams(inputs));
}
