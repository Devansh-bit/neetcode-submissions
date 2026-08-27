class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        rows = len(grid)
        cols = len(grid[0])
        islands = 0
        def dfs(g, row, col):
            if g[row][col] == "0":
                return
            g[row][col] = "0"
            if row+1 < rows:
                dfs(g, row+1, col)
            if col+1 < cols:
                dfs(g, row, col+1)
            if row-1 >= 0:
                dfs(g, row-1, col)
            if col-1 >= 0:
                dfs(g, row, col-1)
             
        for row in range(rows):
            for col in range(cols):
                if grid[row][col] == "1":
                    islands += 1
                    dfs(grid, row, col)
                
                

        return islands