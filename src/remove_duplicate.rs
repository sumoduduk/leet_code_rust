//remove duplicate in array

fn remove_duplicate(nums: &mut Vec<i32>) -> i32 {
    let mut count = 0;
    let mut temp = 0;
    let len = nums.len();

    let mut i = 0;
    while i < len {
        let num = nums[i];

        dbg!(i);
        // match count {
        //     0 => temp = num,
        //     _ => {
        //         nums.remove(i);
        //         count += match temp {
        //             t if t == num => 1,
        //             _ => -1,
        //         };
        //     }
        // }
        //

        if count == 0 {
            temp = num;
        }

        count += match temp {
            t if t == num => {
                nums.remove(i);
                1
            }
            _ => -1,
        };
        dbg!(count);
        dbg!(temp);
        dbg!(num);
        dbg!(&nums);

        i += 1;
    }
    nums.len() as i32
}

pub fn run() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    // let mut nums = vec![0, 0, 1];

    let nums_index = remove_duplicate(&mut nums);

    assert_eq!(5, nums_index);
    assert_eq!(vec![0, 1, 2, 3, 4], nums);
    // assert_eq!(vec![0, 1], nums);
}
