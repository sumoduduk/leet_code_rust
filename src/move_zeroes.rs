fn move_zeroes(nums: &mut Vec<i32>) {
    let len_before = nums.len();

    if len_before == 1 {
        if nums[0] == 0 {
            return;
        }
    }

    nums.retain(|&x| x != 0);

    let total = len_before - nums.len();

    for _ in 0..total {
        nums.push(0);
    }
}

pub fn run() {
    let mut input = vec![0, 1, 0, 3, 12];
    let output = vec![1, 3, 12, 0, 0];

    move_zeroes(&mut input);

    assert_eq!(output, input);

    let mut input = vec![0];
    let output = vec![0];

    move_zeroes(&mut input);

    assert_eq!(output, input);
}
