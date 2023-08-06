fn max_area(height: Vec<i32>) -> i32 {
    let mut first_max = 0;
    let mut second_max = height.len() as i32 - 1;
    let mut area_max = 0;

    while first_max < second_max {
        let new_height = height[first_max as usize].min(height[second_max as usize]);
        let wide = second_max - first_max;
        let new_area = new_height * wide;
        area_max = area_max.max(new_area);

        match (height[first_max as usize], height[second_max as usize]) {
            (first, second) if first < second => first_max += 1,
            (first, second) if first > second => second_max -= 1,
            _ => first_max += 1,
        }
    }

    area_max
}

#[test]
fn max_area_1() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(49, max_area(height));
}
#[test]
fn max_area_2() {
    let height = vec![1, 1];
    assert_eq!(1, max_area(height));
}
#[test]
fn max_area_3() {
    let height = vec![2, 3, 4, 5, 18, 17, 6];
    assert_eq!(17, max_area(height));
}
