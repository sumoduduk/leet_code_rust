fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let mut table = HashMap::with_capacity(nums.len());
    let mut legit = false;

    for (i, num) in nums.iter().enumerate() {
        let j = i as i32;
        let inserted = table.insert(num, j);

        match inserted {
            Some(n) => {
                let total = (n - j).abs();
                if total <= k {
                    legit = true;
                    break;
                }
            }
            None => (),
        }
    }
    legit
}

#[test]
fn test_1() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;

    assert_eq!(true, contains_nearby_duplicate(nums, k))
}

#[test]
fn test_2() {
    let nums = vec![1, 0, 1, 1];
    let k = 1;

    assert_eq!(true, contains_nearby_duplicate(nums, k))
}

#[test]
fn test_3() {
    let nums = vec![1, 2, 3, 1, 2, 3];
    let k = 2;

    assert_eq!(false, contains_nearby_duplicate(nums, k))
}
