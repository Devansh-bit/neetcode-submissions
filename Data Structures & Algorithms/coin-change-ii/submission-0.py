class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        memo = {}
        
        def dfs(i, a):
            # Base Case 1: We hit the exact amount
            if a == 0:
                return 1
                
            # Base Case 2: We overshot, or we ran out of coins to check
            if a < 0 or i == len(coins):
                return 0
                
            if (i, a) in memo:
                return memo[(i, a)]
                
            # Choice 1: Pick the current coin. 
            # We subtract the value, but KEEP the same index 'i' 
            # because we have an infinite supply of this coin.
            pick = dfs(i, a - coins[i])
            
            # Choice 2: Skip the current coin.
            # We keep the amount the same, but move to index 'i + 1'.
            # We will never look at coin 'i' again.
            skip = dfs(i + 1, a)
            
            memo[(i, a)] = pick + skip
            return memo[(i, a)]
            
        return dfs(0, amount)