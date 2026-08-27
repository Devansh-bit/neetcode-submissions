class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        max_area = 0
        rows = len(grid)
        cols = len(grid[0])

        def dfs(grid, row, col) -> int:
            if not((0 <= row < rows) and (0 <= col < cols)):
                return 0
            if grid[row][col] == 0:
                return 0
            grid[row][col] = 0
            
            return 1 + dfs(grid, row+1, col) + dfs(grid, row, col+1) + dfs(grid, row-1, col) + dfs(grid, row, col-1)


        for row in range(rows):
            for col in range(cols):
                if grid[row][col] == 1:
                    max_area = max(max_area, dfs(grid, row, col))

        return max_area