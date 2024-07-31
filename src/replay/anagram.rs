fn is_anagram(s: String, t: String) -> bool {
    let slen = s.len();
    let tlen = t.len();

    if tlen != slen {
        return false;
    }

    let sbyte = s.as_bytes();
    let tbyte = t.as_bytes();

    let mut s_arr: [u16; 26] = [0; 26];
    let mut t_arr: [u16; 26] = [0; 26];

    for i in 0..tlen {
        s_arr[(sbyte[i] - b'a') as usize] += 1;
        t_arr[(tbyte[i] - b'a') as usize] += 1;
    }

    s_arr == t_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_replay_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        let res = is_anagram(s, t);

        assert!(res);
    }

    #[test]
    fn test_anagram_replay_2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        let res = is_anagram(s, t);

        assert!(!res);
    }

    #[test]
    fn test_anagram_replay_3() {
        let s = "ac".to_string();
        let t = "bb".to_string();

        let res = is_anagram(s, t);

        assert!(!res);
    }

    #[test]
    fn test_anagram_replay_4() {
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
        let t = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba".to_string();

        let res = is_anagram(s, t);

        assert!(!res);
    }
}
