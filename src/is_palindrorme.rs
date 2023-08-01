fn is_palindrome(s: String) -> bool {
    let lower = s.to_lowercase();
    let v: Vec<&str> = lower.matches(char::is_alphanumeric).collect();

    let s = v.clone();

    let mut reverse = s.clone();

    reverse.reverse();

    if reverse == v {
        true
    } else {
        false
    }
}

pub fn run() {
    // let s = "A man, a plan, a canal: Panama".to_owned();
    // let s = "race a car".to_owned();
    // let s = " ".to_owned();
    let s = "0P".to_owned();

    let legit = is_palindrome(s);
    assert_eq!(false, legit)
}
