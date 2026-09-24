class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        memo = {}
        def dfs(i, j) -> int:
            if i >= m-1 or j >= n-1:
                return 1
            if (i, j) in memo:
                return memo[i, j]
            
            right = dfs(i+1, j)
            down = dfs(i, j+1)
            memo[i, j] = right+down
            return memo[i, j]
        return dfs(0, 0)