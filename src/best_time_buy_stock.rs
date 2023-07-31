fn buy_n_sell(prices: &Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buying = prices[0];

    for &price in prices.iter() {
        profit = profit.max(price - buying);
        buying = buying.min(price)
    }

    profit
}

pub fn run() {
    let prices = vec![2, 4, 10, 3, 1, 3, 4, 7, 11, 6, 0, 7];
    // let prices = vec![3, 2, 6, 5, 0, 3];
    // let prices = vec![10, 0];

    // let prices = vec![1, 4, 2];
    // let prices = vec![2, 4, 1];
    let profit = buy_n_sell(&prices);
    assert_eq!(10, profit)
}
