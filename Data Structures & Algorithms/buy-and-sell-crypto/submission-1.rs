impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut left_min = prices[0];
        for &price in prices.iter() {
            left_min = left_min.min(price);
            max_profit = max_profit.max(price - left_min);
        }
        max_profit
    }
}
