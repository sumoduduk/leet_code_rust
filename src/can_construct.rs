fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut legit = true;

    let mut mag = magazine.clone();

    for char in ransom_note.chars() {
        let index = mag.find(char);

        match index {
            Some(p) => {
                mag.remove(p);
            }
            None => {
                legit = false;
                break;
            }
        }
    }
    legit
}

pub fn run() {
    let ran_a = "a".to_string();
    let mag_a = "b".to_string();

    assert_eq!(false, can_construct(ran_a, mag_a));

    let ran_b = "aa".to_string();
    let mag_b = "ab".to_string();

    assert_eq!(false, can_construct(ran_b, mag_b));

    let ran_c = "aa".to_string();
    let mag_c = "aab".to_string();

    assert_eq!(true, can_construct(ran_c, mag_c));

    let ran_d = "bg".to_string();
    let mag_d = "efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj".to_string();

    assert_eq!(true, can_construct(ran_d, mag_d));
}
