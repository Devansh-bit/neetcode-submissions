from typing import List

class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        # dp[i] stores the minimum coins to make amount i
        # Initialize with amount + 1 (acts as infinity)
        dp = [amount + 1] * (amount + 1)
        dp[0] = 0

        # Build solutions for every sub-amount from 1 to amount
        for a in range(1, amount + 1):
            for coin in coins:
                if a - coin >= 0:
                    dp[a] = min(dp[a], 1 + dp[a - coin])

        return dp[amount] if dp[amount] != amount + 1 else -1