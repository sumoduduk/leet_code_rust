//search majority in element array
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut vote = 0;
    let mut winner: Option<i32> = None;

    for num in &nums {
        if vote == 0 {
            winner = Some(*num);
        }
        vote += match winner {
            Some(win) if win == *num => 1,
            _ => -1,
        };
    }

    winner.unwrap()
}

pub fn run() {
    let arr = vec![2, 1, 1];

    let win = majority_element(arr);
    dbg!(win);
    assert_eq!(1, win)
}
