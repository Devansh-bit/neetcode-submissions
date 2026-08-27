class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        # Pad grid with 0s on all sides to eliminate boundary checks
        cols = len(grid[0])
        padded = [[0] * (cols + 2)]
        for row in grid:
            padded.append([0] + row + [0])
        padded.append([0] * (cols + 2))

        def dfs(r: int, c: int) -> int:
            if not padded[r][c]:
                return 0
            padded[r][c] = 0
            return 1 + dfs(r + 1, c) + dfs(r - 1, c) + dfs(r, c + 1) + dfs(r, c - 1)

        max_area = 0
        for r in range(1, len(padded) - 1):
            row_r = padded[r]
            for c in range(1, cols + 1):
                if row_r[c]:
                    area = dfs(r, c)
                    if area > max_area:
                        max_area = area

        return max_area