class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        memo = {}
        
        def dfs(i, j) -> int:
            # 1. Out of bounds (Failure)
            if i >= m or j >= n:
                return 0
                
            # 2. Reached the exact destination (Success)
            if i == m - 1 and j == n - 1:
                return 1
                
            if (i, j) in memo:
                return memo[i, j]
            
            # 3. Explore both physical choices
            right = dfs(i, j + 1)
            down = dfs(i + 1, j)
            
            memo[i, j] = right + down
            return memo[i, j]
            
        return dfs(0, 0)