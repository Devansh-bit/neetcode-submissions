class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        rows, cols = len(matrix), len(matrix[0])
        memo = {}
        def dfs(i, j) -> int:
            if not (0 <= i < rows) or not (0 <= j < cols):
                return 0
            if (i, j) in memo:
                return memo[i, j]
            
            curr_max = 0
            for (idx, di, dj) in [(0, 0, 1), (1, 0, -1), (2, 1, 0), (3, -1, 0)]:
                ni, nj = i + di, j + dj
                if (0 <= ni < rows) and (0 <= nj < cols) and matrix[ni][nj] > matrix[i][j]:
                    curr_max = max(curr_max, dfs(ni, nj))
            memo[i, j] = 1 + curr_max
            return memo[i, j]
        
        curr_max = 0
        for row in range(rows):
            for col in range(cols):
                curr_max = max(curr_max, dfs(row, col))
        
        return curr_max