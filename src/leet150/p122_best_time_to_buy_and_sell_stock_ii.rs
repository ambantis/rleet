/// 122. Best Time to Buy and Sell Stock II
///
/// You are given an array `prices` where `prices[i]` is the price of a stock on
/// day `i`. Each day you may buy the stock, sell it, or do nothing, but you can
/// hold at most one share at a time. Selling and buying again on the same day is
/// allowed. Return the maximum profit you can achieve.
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profits = 0;

    prices
        .iter()
        .zip(prices[1..].iter())
        .for_each(|(buy, sell)| {
            let profit = sell - buy;
            if profit > 0 {
                profits += profit;
            }
        });

    profits
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
