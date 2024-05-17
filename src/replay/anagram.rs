fn is_anagram(s: String, t: String) -> bool {
    let mut s_bytes = s.as_bytes().to_vec();
    let mut t_bytes = t.as_bytes().to_vec();

    s_bytes.sort();
    t_bytes.sort();

    s_bytes == t_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        let res = is_anagram(s, t);

        assert_eq!(res, true);
    }

    #[test]
    fn test_anagram_2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        let res = is_anagram(s, t);

        assert_eq!(res, false);
    }
}
