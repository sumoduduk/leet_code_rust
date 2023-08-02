fn is_isomorphic(s: String, t: String) -> bool {
    use std::collections::{HashMap, HashSet};
    let s_len = s.len();
    let t_len = t.len();

    if s_len != t_len {
        return false;
    }

    let byte = t.into_bytes();

    let mut map: HashMap<char, u8> = HashMap::with_capacity(s_len);
    let mut set_map: HashSet<u8> = HashSet::with_capacity(t_len);

    let mut legit = false;

    for (i, char) in s.char_indices() {
        let val = &byte[i];
        let inserted = map.insert(char, *val);

        match inserted {
            Some(alp) => {
                if alp != *val {
                    legit = false;
                    break;
                } else {
                    legit = true;
                }
            }
            None => {
                let set_inserted = set_map.insert(*val);

                if !set_inserted {
                    legit = false;
                    break;
                } else {
                    legit = true;
                }
            }
        }
    }
    legit
}

pub fn run() {
    let s = "egg".to_string();
    let t = "add".to_string();

    assert_eq!(true, is_isomorphic(s, t));

    let s = "foo".to_string();
    let t = "bar".to_string();

    assert_eq!(false, is_isomorphic(s, t));

    let s = "paper".to_string();
    let t = "title".to_string();

    assert_eq!(true, is_isomorphic(s, t));

    let s = "badc".to_string();
    let t = "baba".to_string();

    assert_eq!(false, is_isomorphic(s, t));
}
