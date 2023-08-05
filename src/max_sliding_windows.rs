fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    if nums.len() as i32 == k {
        let max = nums.iter().max().unwrap();

        return vec![*max];
    }

    let max_chunks = nums.windows(k as usize);

    let mut arr = Vec::with_capacity(nums.len());

    for chunk in max_chunks {
        let mut mini = i32::MIN;
        let mut maxi = VecDeque::from(chunk.to_vec());
        for i in 0..maxi.len() {
            if maxi.len() == 1 {
                arr.push(maxi[0].max(mini));
                break;
            }

            if maxi[0] > mini {
                mini = maxi[i];
            }

            maxi.pop_front();
        }
    }

    arr
}

// #[test]
// fn max_sliding_window_test_1() {
//     let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
//
//     let k = 3;
//     let output = vec![3, 3, 5, 5, 6, 7];
//
//     assert_eq!(output, max_sliding_window(nums, k));
// }
//
// #[test]
// fn max_sliding_window_test_2() {
//     let nums = vec![1];
//
//     let k = 1;
//     let output = vec![1];
//
//     assert_eq!(output, max_sliding_window(nums, k));
// }
