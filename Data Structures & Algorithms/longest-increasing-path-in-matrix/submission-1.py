from collections import deque

class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        rows, cols = len(matrix), len(matrix[0])
        dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)]

        # Step 1: indegree = number of smaller neighbors.
        indeg = [[0] * cols for _ in range(rows)]
        for i in range(rows):
            for j in range(cols):
                for di, dj in dirs:
                    ni, nj = i + di, j + dj
                    if 0 <= ni < rows and 0 <= nj < cols and matrix[ni][nj] < matrix[i][j]:
                        indeg[i][j] += 1

        # Step 2: start with cells that have no smaller neighbor.
        q = deque((i, j) for i in range(rows) for j in range(cols) if indeg[i][j] == 0)

        # Steps 3 to 5: remove one layer at a time.
        layers = 0
        while q:
            layers += 1
            for _ in range(len(q)):
                i, j = q.popleft()
                for di, dj in dirs:
                    ni, nj = i + di, j + dj
                    if 0 <= ni < rows and 0 <= nj < cols and matrix[ni][nj] > matrix[i][j]:
                        indeg[ni][nj] -= 1
                        if indeg[ni][nj] == 0:
                            q.append((ni, nj))

        return layers