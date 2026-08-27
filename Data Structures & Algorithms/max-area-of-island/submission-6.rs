impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut max_area = 0;

        fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, rows: usize, cols: usize) -> i32 {
            if r >= rows || c >= cols || grid[r][c] == 0 {
                return 0;
            }

            // Sink the island
            grid[r][c] = 0;
            let mut area = 1;

            // Down
            if r + 1 < rows {
                area += dfs(grid, r + 1, c, rows, cols);
            }
            // Up
            if r > 0 {
                area += dfs(grid, r - 1, c, rows, cols);
            }
            // Right
            if c + 1 < cols {
                area += dfs(grid, r, c + 1, rows, cols);
            }
            // Left
            if c > 0 {
                area += dfs(grid, r, c - 1, rows, cols);
            }

            area
        }

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    let area = dfs(&mut grid, r, c, rows, cols);
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }
}