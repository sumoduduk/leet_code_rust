//merge 2 array
//length array 1 m, and array 2 n
//non-decreasing
//array2 => array1
//
//
//
//
#![allow(dead_code)]

fn merge(arr1: &mut Vec<i32>, m: i32, arr2: &mut Vec<i32>, n: i32) {
    let total = m + n;

    for elem in arr2.iter() {
        arr1.push(*elem)
    }

    arr1.retain(|&x| x != 0);

    let len = arr1.len() as i32;
    if len < total {
        let minus = total - len;
        for _ in 0..minus {
            arr1.push(0);
        }
    }

    arr1.sort();

    assert_eq!(vec![-1, 0, 0, 1, 2, 2, 3, 3, 3], *arr1)
}

pub fn run() {
    let mut arr1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
    let mut arr2 = vec![1, 2, 2];

    let m = 6;
    let n = 3;

    merge(&mut arr1, m, &mut arr2, n);

    // dbg!(&hasil);
    // assert_eq!(vec![1, 2, 3, 4, 0, 5, 0, 2, 10, 8, 4, 0], *hasil)
}
