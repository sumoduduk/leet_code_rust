fn cum_rev(mut arr: [i8; 6]) -> [i8; 6] {
    let mut i = arr.len() - 2;
    loop {
        arr[i] = arr[i] + arr[i + 1];
        if i == 0 {
            break;
        } else {
            i = i - 1;
        }
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cum_rev() {
        let arr = [4, 6, 1, 3, 1, 5];
        let target = [20, 16, 10, 9, 6, 5];

        let res = cum_rev(arr);
        dbg!(res);
        assert_eq!(res, target);
    }
}
