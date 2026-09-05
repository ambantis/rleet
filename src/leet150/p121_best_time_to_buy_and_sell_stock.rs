/// 121. Best Time to Buy and Sell Stock
///
/// You are given an array `prices` where `prices[i]` is the price of a stock on
/// day `i`. You may buy on one day and sell on a later day. Find the maximum
/// profit you can get. If no profit is possible, return 0.
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len: usize = prices.len();
    let mut max_from_right = prices.clone();
    for i in (1..len).rev() {
        max_from_right[i - 1] = max_from_right[i - 1].max(max_from_right[i]);
    }

    prices
        .iter()
        .zip(&max_from_right)
        .map(|(buy, sell)| sell - buy)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_day() {
        assert_eq!(max_profit(vec![5]), 0);
    }

    #[test]
    fn rising_prices() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn flat_prices() {
        assert_eq!(max_profit(vec![3, 3, 3, 3]), 0);
    }
}
