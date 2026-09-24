class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        memo = {}
        def dfs(i, is_holding) -> int:
            if i >= len(prices):
                return 0
            if (i, is_holding) in memo:
                return memo[i, is_holding]
            
            if is_holding:
                skip = dfs(i+1, is_holding)
                sell = prices[i] + dfs(i+2, False)
                memo[i, is_holding] = max(skip, sell)
                return memo[i, is_holding]
            
            else:
                skip = dfs(i+1, is_holding)
                buy = - prices[i] + dfs(i+1, True)
                memo[i, is_holding] = max(skip, buy)
                return memo[i, is_holding]
        return dfs(0, False)