class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        if not grid or not grid[0]:
            return 0
        
        rows, cols = len(grid), len(grid[0])
        total_cells = rows * cols
        
        # Check if there are any land cells at all
        if not any(1 in row for row in grid):
            return 0

        parent = list(range(total_cells))
        size = [0] * total_cells
        max_size = 0

        def find(i: int) -> int:
            path = []
            while parent[i] != i:
                path.append(i)
                i = parent[i]
            for node in path:
                parent[node] = i
            return i

        def union(i: int, j: int):
            nonlocal max_size
            root_i = find(i)
            root_j = find(j)
            if root_i != root_j:
                if size[root_i] < size[root_j]:
                    root_i, root_j = root_j, root_i
                parent[root_j] = root_i
                size[root_i] += size[root_j]
                if size[root_i] > max_size:
                    max_size = size[root_i]

        for r in range(rows):
            row_data = grid[r]
            row_offset = r * cols
            for c in range(cols):
                if row_data[c] == 1:
                    idx = row_offset + c
                    size[idx] = 1
                    if max_size == 0:
                        max_size = 1
                    
                    # Connect with left neighbor
                    if c > 0 and row_data[c - 1] == 1:
                        union(idx, idx - 1)
                    
                    # Connect with top neighbor
                    if r > 0 and grid[r - 1][c] == 1:
                        union(idx, idx - cols)

        return max_size