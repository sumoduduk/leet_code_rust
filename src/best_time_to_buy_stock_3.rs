fn buy_n_sell(prices: Vec<i32>) -> i32 {
    let mut profits: Vec<i32> = Vec::new();
    let mut buying = prices[0];

    for &price in prices.iter() {
        buying = buying.min(price);
        if price > buying {
            profits.push(price - buying);
            buying = price;
        }
    }

    profits.sort();

    let len = profits.len();

    match len {
        0 => 0,
        1 => profits[0],
        _ => profits[len - 1] + profits[len - 2],
    }
}

pub fn run() {
    // let prices = vec![2, 4, 10, 3, 1, 3, 4, 7, 11, 6, 0, 7];
    // let prices = vec![3, 2, 6, 5, 0, 3];
    // let prices = vec![10, 0];

    // let prices = vec![1, 4, 2];
    // let prices = vec![7, 1, 5, 3, 6, 4];
    let prices = vec![1, 2, 3, 4, 5];
    // let prices = vec![7, 6, 4, 3, 1];
    // let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];

    let profit = buy_n_sell(prices);
    assert_eq!(2, profit)
}
