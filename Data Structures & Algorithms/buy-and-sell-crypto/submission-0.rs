impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut i: i32 = n as i32 - 1;
        let mut max_profit = 0;
        let mut right_max = 0;
        while i > -1 {
            right_max = right_max.max(prices[i as usize]);
            max_profit = max_profit.max(right_max - prices[i as usize]);
            i -= 1;
            
        }
        max_profit
    }
}
