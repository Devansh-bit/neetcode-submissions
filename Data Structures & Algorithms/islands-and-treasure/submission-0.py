from collections import deque
from typing import List

class Solution:
    def islandsAndTreasure(self, grid: List[List[int]]) -> None:
        rows, cols = len(grid), len(grid[0])
        queue = deque()

        # Step 1: Add all treasure chests (0) to the queue
        for r in range(rows):
            for c in range(cols):
                if grid[r][c] == 0:
                    queue.append((r, c))

        # Step 2: Multi-source BFS
        while queue:
            r, c = queue.popleft()

            for dr, dc in [(0, 1), (1, 0), (-1, 0), (0, -1)]:
                nr, nc = r + dr, c + dc

                # Only move to valid empty rooms (INF = 2147483647)
                if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == 2147483647:
                    grid[nr][nc] = grid[r][c] + 1
                    queue.append((nr, nc))