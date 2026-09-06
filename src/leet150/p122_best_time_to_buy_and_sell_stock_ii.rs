/// 122. Best Time to Buy and Sell Stock II
///
/// You are given an array `prices` where `prices[i]` is the price of a stock on
/// day `i`. Each day you may buy the stock, sell it, or do nothing, but you can
/// hold at most one share at a time. Selling and buying again on the same day is
/// allowed. Return the maximum profit you can achieve.
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices.windows(2).map(|w| 0.max(w[1] - w[0])).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_day() {
        assert_eq!(max_profit(vec![5]), 0);
    }

    #[test]
    fn flat_prices() {
        assert_eq!(max_profit(vec![3, 3, 3, 3]), 0);
    }

    #[test]
    fn peaks_and_valleys() {
        assert_eq!(max_profit(vec![1, 5, 2, 8, 3, 9]), 16);
    }
}
