fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    use std::cmp::Ordering::*;

    let mut legit = false;
    let len = matrix.len();

    if len <= 2 {
        for num in &matrix {
            if num.contains(&target) {
                legit = true;
                break;
            }
        }

        return legit;
    }

    let middle_index = len / 2;
    let left_middle_index = middle_index - 1;
    let right_middle_index = middle_index + 1;

    let midde_num_first = &matrix[middle_index][0];
    let midde_num_left = &matrix[left_middle_index][0];
    let midde_num_right = &matrix[right_middle_index][0];
    match target.cmp(&midde_num_first) {
        Less => match target.cmp(&midde_num_left) {
            Equal => legit = true,
            Greater => legit = matrix[left_middle_index].contains(&target),
            Less => {
                for i in 0..left_middle_index {
                    if target >= matrix[i][0] && target < matrix[i + 1][0] {
                        legit = matrix[i].contains(&target);
                    }
                }
            }
        },
        Equal => legit = true,
        Greater => match target.cmp(&midde_num_right) {
            Equal => legit = true,
            Less => legit = matrix[middle_index].contains(&target),
            Greater => {
                if matrix[right_middle_index].contains(&target) {
                    legit = true
                } else if matrix[len - 1].contains(&target) {
                    legit = true
                } else {
                    for i in right_middle_index..len - 1 {
                        if target >= matrix[i][0] && target < matrix[i + 1][0] {
                            legit = matrix[i].contains(&target);
                        }
                    }
                }
            }
        },
    }

    legit
}

#[test]
fn search_matrix_1() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;

    assert_eq!(true, search_matrix(matrix, target));
}

#[test]
fn search_matrix_2() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 13;

    assert_eq!(false, search_matrix(matrix, target));
}

#[test]
fn search_matrix_3() {
    let matrix = vec![
        vec![-9, -8, -8],
        vec![-5, -3, -2],
        vec![0, 2, 2],
        vec![4, 6, 8],
    ];
    let target = 15;

    assert_eq!(false, search_matrix(matrix, target));
}

#[test]
fn search_matrix_4() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]];
    let target = 15;

    assert_eq!(false, search_matrix(matrix, target));
}

#[test]
fn search_matrix_5() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]];
    let target = 34;

    assert_eq!(true, search_matrix(matrix, target));
}

#[test]
fn search_matrix_6() {
    let matrix = vec![
        vec![-9, -8, -8],
        vec![-5, -3, -2],
        vec![0, 2, 2],
        vec![4, 6, 8],
    ];
    let target = 8;

    assert_eq!(true, search_matrix(matrix, target));
}
