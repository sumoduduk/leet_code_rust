fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }

    let mut t = t.clone();

    let mut contain_all = false;
    let mut s_index = 0;

    for (i, ch) in s.char_indices() {
        let t_index = t.find(ch);

        match t_index {
            Some(p) => {
                if p < s_index {
                    contain_all = false;
                    break;
                } else if p == s_index {
                    if i > 0 {
                        t.remove(p);
                    }
                } else {
                    s_index = p;
                    contain_all = true;
                }
            }
            None => {
                contain_all = false;
                break;
            }
        }
    }

    contain_all
}

pub fn run() {
    // let s = "cba".to_owned();
    // let t = "ahbgdc".to_owned();

    // let s = "aaaaaa".to_owned();
    // let t = "bbaaaa".to_owned();

    let s = "ab".to_string();
    let t = "baab".to_string();

    // let s = "abc".to_owned();
    // let t = "acb".to_owned();

    // let s = "bb".to_owned();
    //
    // let t = "ahbgdc".to_owned();
    // let g = s.find('b');
    // dbg!(g);
    //
    assert_eq!(true, is_subsequence(s, t));
}
