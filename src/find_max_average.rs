fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let len = nums.len();

    let mut maxi = f64::MIN;
    let mut sum = 0;
    let divider = k as f64;

    for i in 0..len {
        let num = nums[i];
        match i as i32 {
            index if index < k => {
                sum += num;
            }
            _ => {
                let average = sum as f64 / divider;
                maxi = maxi.max(average);

                sum += num - nums[i - k as usize];
            }
        }
    }

    maxi = maxi.max(sum as f64 / divider);

    maxi
}

#[test]
fn max_average_easy_1() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;

    assert_eq!(12.75000, find_max_average(nums, k));
}
#[test]

fn max_average_easy_2() {
    let nums = vec![5];
    let k = 1;

    assert_eq!(5.0000, find_max_average(nums, k));
}
