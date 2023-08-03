fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut val: Vec<i32> = Vec::with_capacity(nums.len());
    let mut rev = nums.clone();
    rev.reverse();

    nums.val
}

pub fn run() {
    let nums = vec![8, 7, 11, 2];
    let target = 9;
    let output = vec![1, 3];

    assert_eq!(output, two_sum(nums, target));
}
