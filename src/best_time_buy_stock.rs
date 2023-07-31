fn buy_n_sell(nums: &Vec<i32>) -> i32 {
    let mut index_max: usize = 0;
    let mut index_min: usize = 0;
    let mut max: i32 = nums[0];
    let mut min: i32 = nums[0];
    let len = nums.len();

    for (i, num) in nums.iter().enumerate() {
        let max_inner = max.max(*num);
        if max != max_inner {
            max = max_inner;
            index_max = i;
        }

        let min_inner = min.min(*num);
        if min != min_inner {
            if index_max > i {
                min = min_inner;
                index_min = i;
            }
        }

        dbg!(i);
        dbg!(num);
        dbg!(index_min);
        dbg!(min);
        dbg!(index_max);
        dbg!(max);
        println!("\n");
    }

    if index_min < index_max {
        max - min
    } else {
        0
    }
}

pub fn run() {
    let nums = vec![2, 4, 10, 3, 1, 3, 4, 7, 11, 6, 0, 7];
    // let nums = vec![3, 2, 6, 5, 0, 3];

    // let nums = vec![1, 4, 2];
    dbg!(&nums);
    let profit = buy_n_sell(&nums);
    assert_eq!(10, profit)
}
