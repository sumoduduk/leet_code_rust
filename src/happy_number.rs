pub fn is_happy(mut n: i32) -> bool {
    use std::collections::HashSet;
    let mut table = HashSet::new();

    let mut legit = false;

    while !legit {
        let total: u32 = n
            .to_string()
            .chars()
            .map(|s| s.to_digit(10).unwrap().pow(2))
            .sum();

        if total == 1 {
            legit = true;
        }

        let inserted = table.insert(total);

        match inserted {
            true => n = total as i32,
            false => break,
        }
    }

    legit
}

#[test]
fn test_1() {
    assert_eq!(true, is_happy(19));
}

#[test]
fn test_2() {
    assert_eq!(false, is_happy(2));
}

#[test]
fn all_happy() {
    let num = [
        1, 7, 10, 13, 19, 23, 28, 31, 32, 44, 49, 68, 70, 79, 82, 86, 91, 94, 97, 100,
    ];

    for n in num {
        assert_eq!(true, is_happy(n));
    }
}

#[test]
fn all_not_happy() {
    let num = [
        2, 3, 4, 5, 6, 8, 9, 11, 12, 14, 15, 16, 17, 18, 20, 21, 22, 24, 25, 26,
    ];

    for n in num {
        assert_eq!(false, is_happy(n));
    }
}
