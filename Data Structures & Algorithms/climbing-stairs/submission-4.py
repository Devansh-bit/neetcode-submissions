class Solution:
    def climbStairs(self, n: int) -> int:
        dp = {}
        def dfs(n):
            if n <= 1:
                return 1
            if n in dp:
                return dp[n]
            res = dfs(n-1) + dfs(n-2)
            dp[n] = res
            return res
        return dfs(n)
