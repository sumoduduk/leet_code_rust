pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum = 0;

    let total = nums.iter().sum::<i32>();

    let mut target = -1;

    for (i, num) in nums.iter().enumerate() {
        match (sum, num) {
            (s, n) if s == total - s - n => {
                target = i as i32;
                break;
            }
            _ => sum += num,
        }
    }
    target
}

#[test]
fn pivot_index_1() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    let output = 3;

    assert_eq!(output, pivot_index(nums));
}

#[test]
fn pivot_index_2() {
    let nums = vec![1, 2, 3];
    let output = -1;

    assert_eq!(output, pivot_index(nums));
}

#[test]
fn pivot_index_3() {
    let nums = vec![2, 1, -1];
    let output = 0;

    assert_eq!(output, pivot_index(nums));
}
#[test]
fn pivot_index_4() {
    let nums = vec![-1, -1, -1, -1, -1, 0];
    let output = 2;

    assert_eq!(output, pivot_index(nums));
}

#[test]
fn pivot_index_5() {
    let nums = vec![-1, -1, 0, 0, -1, -1];

    let output = 2;

    assert_eq!(output, pivot_index(nums));
}
