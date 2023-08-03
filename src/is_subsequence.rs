fn is_subsequence(s: String, t: String) -> bool {
    let mut t_index = 0;

    for c in s.chars() {
        let finder = &t[t_index..t.len()];

        match finder.find(c) {
            Some(i) => t_index += i + 1,
            None => return false,
        }
    }

    true
}

pub fn run() {
    // let s = "cba".to_owned();
    // let t = "ahbgdc".to_owned();

    let s = "aaaaaa".to_owned();
    let t = "bbaaaa".to_owned();

    // let s = "ab".to_string();
    // let t = "baab".to_string();

    // let s = "abc".to_owned();
    // let t = "acb".to_owned();

    // let s = "bb".to_owned();
    //
    // let t = "ahbgdc".to_owned();
    // let g = s.find('b');
    // dbg!(g);
    //
    assert_eq!(false, is_subsequence(s, t));
}
