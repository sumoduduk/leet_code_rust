fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    if nums.len() == 0 {
        return 0;
    }
    let mut arr = nums.clone();
    arr.sort();
    dbg!(&arr);

    let num_len = nums.len();

    let mut table_set = HashSet::with_capacity(num_len);

    let mut new_arr = Vec::with_capacity(num_len);
    let mut tmp_num = arr[0];
    new_arr.push(tmp_num);

    let mut max_consec = 0;

    for num in arr {
        match (num, tmp_num) {
            (n, t) if n == t => continue,
            (n, t) if n - t == 1 => {
                dbg!(n - t);
                new_arr.push(n);
                tmp_num = num;
            }
            (n, t) if n - t != 1 => {
                table_set.insert(new_arr.len());
                new_arr.clear();
                tmp_num = num;
                new_arr.push(num);
            }
            (_, _) => (),
        }
    }

    if new_arr.len() > 0 {
        table_set.insert(new_arr.len());
    }

    for len in table_set.into_iter() {
        max_consec = max_consec.max(len);
    }
    max_consec as i32
}

pub fn run() {
    // let input = vec![100, 4, 200, 1, 3, 2];
    // let output = 4;
    //
    // assert_eq!(output, longest_consecutive(input));
    //
    // let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    // let output = 9;
    //
    // assert_eq!(output, longest_consecutive(input));

    let input = vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6];

    let output = 7;
    assert_eq!(output, longest_consecutive(input));
}
