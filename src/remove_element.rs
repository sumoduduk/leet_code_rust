//remove elemen in array == value
//return the len reminder

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

pub fn run() {
    let mut arr = vec![0, 1, 2, 2, 3, 0, 4, 2];

    let val = 2;

    assert_eq!(5, remove_element(&mut arr, val))
}
