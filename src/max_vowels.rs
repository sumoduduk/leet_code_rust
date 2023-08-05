fn max_vowels(s: String, k: i32) -> i32 {
    use std::collections::VecDeque;
    let vowel = ['a', 'i', 'u', 'e', 'o'];

    let mut maxi = 0;

    let mut vowel_max = 0;

    let mut deq = VecDeque::with_capacity(k as usize);

    for (i, char) in s.char_indices() {
        match i as i32 {
            index if index < k => {
                deq.push_back(char);
                if vowel.contains(&char) {
                    maxi += 1;
                }
            }
            _ => {
                vowel_max = vowel_max.max(maxi);
                let front_char = deq.pop_front().unwrap();
                if vowel.contains(&front_char) {
                    maxi -= 1;
                }
                deq.push_back(char);

                if vowel.contains(&char) {
                    maxi += 1;
                }
            }
        }
    }
    vowel_max = vowel_max.max(maxi);
    vowel_max
}

#[test]
fn max_vowels_1() {
    let s = "abciiidef".to_string();
    let k = 3;
    assert_eq!(3, max_vowels(s, k));
}
#[test]
fn max_vowels_2() {
    let s = "aiueo".to_string();
    let k = 2;
    assert_eq!(2, max_vowels(s, k));
}
#[test]
fn max_vowels_3() {
    let s = "leetcode".to_string();
    let k = 3;
    assert_eq!(2, max_vowels(s, k));
}
