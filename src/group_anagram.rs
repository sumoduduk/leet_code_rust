struct Leet {}
impl Leet {
    fn sum_bytes(str: &str) -> u16 {
        str.as_bytes().iter().map(|&s| s as u16).sum()
    }

    fn dedup_sum(vec: &Vec<String>) -> (Vec<u16>, usize) {
        let mut val: Vec<u16> = vec.iter().map(|s| Self::sum_bytes(&s)).collect();
        val.sort();
        val.dedup();
        let len = { val.len() };

        (val, len)
    }

    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let (arr, len) = Self::dedup_sum(&strs);
        let mut val: Vec<Vec<String>> = Vec::with_capacity(len);

        let mut i = 0;
        while i < len {
            val.push(Vec::new());
            strs.iter().for_each(|s| {
                if Self::sum_bytes(&s) == arr[i] {
                    val[i].push(s.to_string())
                }
            });
            i += 1;
        }
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

    let duh = Leet::sum_bytes("duh");
    let ill = Leet::sum_bytes("ill");

    dbg!(duh);
    dbg!(ill);

    assert_eq!(output, Leet::group_anagrams(inputs));
}
