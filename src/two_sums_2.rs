fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    if numbers.len() == 2 {
        if numbers[0] + numbers[1] == target {
            return vec![1, 2];
        }
    }

    let mut table: HashMap<i32, i32> = HashMap::with_capacity(numbers.len());

    let mut val: Vec<i32> = Vec::with_capacity(2);

    for (i, &num) in numbers.iter().enumerate() {
        let tmp = target - &num;
        let j = i as i32;

        let key_valuse = table.get_key_value(&tmp);

        match key_valuse {
            Some((_x, y)) => val = vec![*y, j + 1],
            _ => (),
        }

        table.insert(num, j + 1);
    }

    val
}

pub fn run() {
    let numbers = vec![2, 7, 11, 15];
    let target = 9;
    let output = vec![1, 2];

    assert_eq!(output, two_sum(numbers, target));

    let numbers = vec![2, 3, 4];
    let target = 6;
    let output = vec![1, 3];

    assert_eq!(output, two_sum(numbers, target));

    let numbers = vec![3, 3];
    let target = 6;
    let output = vec![1, 2];

    assert_eq!(output, two_sum(numbers, target));

    let numbers = vec![-1, 0];
    let target = -1;
    let output = vec![1, 2];

    assert_eq!(output, two_sum(numbers, target));

    let numbers = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 7];
    let target = 11;
    let output = vec![13, 14];

    assert_eq!(output, two_sum(numbers, target));
}
