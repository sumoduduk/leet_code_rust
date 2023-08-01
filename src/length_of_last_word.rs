pub fn length_of_last_word(s: String) -> i32 {
    let trim: Vec<_> = s.split_ascii_whitespace().collect();

    let s_len = trim.len();

    let last = trim[s_len - 1];

    last.len() as i32
}

pub fn run() {
    // let s ="Hello World".to_owned();
    // let s = "   fly me   to   the moon  ".to_owned();
    let s = "luffy is still joyboy".to_owned();
    // let s ="Hello World".to_owned();

    let value = length_of_last_word(s);

    assert_eq!(6, value);
}
