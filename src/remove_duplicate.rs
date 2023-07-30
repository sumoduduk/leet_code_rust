//remove duplicate in array

fn remove_duplicate(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    return nums.len() as i32;
}

pub fn run() {
    // let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let mut nums = vec![0, 0, 1];

    let nums_index = remove_duplicate(&mut nums);

    assert_eq!(2, nums_index);
    // assert_eq!(vec![0, 1, 2, 3, 4], nums);
    assert_eq!(vec![0, 1], nums);
}
