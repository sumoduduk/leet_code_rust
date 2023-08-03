fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut sv = s.into_bytes();
    let mut tv = t.into_bytes();

    sv.sort();
    tv.sort();

    if sv == tv {
        true
    } else {
        false
    }
}

pub fn run() {
    let s = "anagram".to_string();
    let t = "nagaram".to_string();

    let val = is_anagram(s, t);

    assert_eq!(true, val);

    let s = "rat".to_string();
    let t = "cat".to_string();

    let val = is_anagram(s, t);

    assert_eq!(true, val);
}
