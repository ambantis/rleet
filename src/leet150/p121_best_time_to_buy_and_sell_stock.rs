/// 121. Best Time to Buy and Sell Stock
///
/// You are given an array `prices` where `prices[i]` is the price of a stock on
/// day `i`. You may buy on one day and sell on a later day. Find the maximum
/// profit you can get. If no profit is possible, return 0.
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cheapest: i32 = i32::MAX;
    let mut result = 0;
    for price in prices.iter() {
        cheapest = cheapest.min(*price);
        result = result.max(price - cheapest);
    }

    result
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
