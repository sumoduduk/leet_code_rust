fn buy_n_sell(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buying = prices[0];

    for &price in prices.iter() {
        buying = buying.min(price);
        if price <= buying {
            profit += 0;
        } else {
            profit += price - buying;
            buying = price;
        }
    }

    profit
}

pub fn run() {
    // let prices = vec![2, 4, 10, 3, 1, 3, 4, 7, 11, 6, 0, 7];
    let prices = vec![3, 2, 6, 5, 0, 3];
    // let prices = vec![10, 0];

    // let prices = vec![1, 4, 2];
    // let prices = vec![7, 1, 5, 3, 6, 4];
    // let prices = vec![1, 2, 3, 4, 5];
    // let prices = vec![7, 6, 4, 3, 1];
    // let prices = vec![7, 6, 4, 3, 1];

    let profit = buy_n_sell(prices);
    assert_eq!(7, profit)
}
