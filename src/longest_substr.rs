fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::VecDeque;
    let mut max_length = 0;
    let mut char_set: VecDeque<char> = VecDeque::new();

    for c in s.chars() {
        while char_set.contains(&c) {
            char_set.pop_front();
        }

        char_set.push_back(c);
        max_length = max_length.max(char_set.len());
    }

    max_length as i32
}

#[test]
fn longest_str_1() {
    let s = "abcabcbb".to_string();

    assert_eq!(3, length_of_longest_substring(s));
}
#[test]
fn longest_str_2() {
    let s = "bbbbb".to_string();

    assert_eq!(1, length_of_longest_substring(s));
}
#[test]
fn longest_str_3() {
    let s = "pwwkew".to_string();

    assert_eq!(3, length_of_longest_substring(s));
}
#[test]
fn longest_str_4() {
    let s = "dvdf".to_string();

    assert_eq!(3, length_of_longest_substring(s));
}
