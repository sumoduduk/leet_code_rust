fn longest_subarray(nums: Vec<i32>) -> i32 {
    if !nums.contains(&0) {
        return nums.len() as i32 - 1;
    }

    let mut maxi = 0;
    let mut sub_maxi = 0;
    let mut sub_before = 0;

    for num in nums {
        match num {
            number if number == 0 => {
                sub_before = sub_maxi;
                sub_maxi = 0;
            }
            _ => {
                sub_maxi += 1;
                maxi = maxi.max(sub_maxi + sub_before);
            }
        }
    }

    maxi
}

#[test]
fn longest_subarray_1() {
    let nums = vec![1, 1, 0, 1];
    assert_eq!(3, longest_subarray(nums));
}

#[test]
fn longest_subarray_2() {
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    assert_eq!(5, longest_subarray(nums));
}

#[test]
fn longest_subarray_3() {
    let nums = vec![1, 1, 1];
    assert_eq!(2, longest_subarray(nums));
}
