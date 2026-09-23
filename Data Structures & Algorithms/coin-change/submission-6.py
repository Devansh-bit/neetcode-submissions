class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        dp = [amount+1]*(amount+1)
        dp[0] = 0
        for a in range(1, amount+1):
            for coin in coins:
                rem = a - coin
                if rem >= 0:
                    dp[a] = min(dp[rem]+1, dp[a])
        return dp[amount] if dp[amount] < amount+1 else -1