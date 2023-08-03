fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    if nums.len() == 2 {
        if nums[0] + nums[1] == target {
            return vec![0, 1];
        }
    }

    let mut table: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

    let mut val: Vec<i32> = Vec::with_capacity(2);

    for (i, &num) in nums.iter().enumerate() {
        let tmp = target - &num;
        let j = i as i32;

        let key_valuse = table.get_key_value(&tmp);

        match key_valuse {
            Some((_x, y)) => val = vec![*y, j],
            _ => (),
        }

        table.insert(num, j);
    }

    val
}

fn cast(num: i32, target: i32) -> [i32; 2] {
    let sum = target - num;

    let tmp = [num, sum];
    tmp
}

pub fn run() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let output = vec![0, 1];

    assert_eq!(output, two_sum(nums, target));

    let nums = vec![3, 2, 4];
    let target = 6;
    let output = vec![1, 2];

    assert_eq!(output, two_sum(nums, target));

    let nums = vec![3, 3];
    let target = 6;
    let output = vec![0, 1];

    assert_eq!(output, two_sum(nums, target));

    let nums = vec![1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 7, 1, 1, 1, 1, 1];
    let target = 11;
    let output = vec![5, 11];

    assert_eq!(output, two_sum(nums, target));
}
