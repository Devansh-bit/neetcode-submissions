impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut max_area = 0;
        
        // Use a fixed-size stack on the stack frame (max island size is rows * cols <= 2500)
        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(rows * cols);

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    grid[r][c] = 0;
                    let mut area = 0;
                    stack.push((r, c));

                    while let Some((cr, cc)) = stack.pop() {
                        area += 1;

                        // Down
                        if cr + 1 < rows && grid[cr + 1][cc] == 1 {
                            grid[cr + 1][cc] = 0;
                            stack.push((cr + 1, cc));
                        }
                        // Up
                        if cr > 0 && grid[cr - 1][cc] == 1 {
                            grid[cr - 1][cc] = 0;
                            stack.push((cr - 1, cc));
                        }
                        // Right
                        if cc + 1 < cols && grid[cr][cc + 1] == 1 {
                            grid[cr][cc + 1] = 0;
                            stack.push((cr, cc + 1));
                        }
                        // Left
                        if cc > 0 && grid[cr][cc - 1] == 1 {
                            grid[cr][cc - 1] = 0;
                            stack.push((cr, cc - 1));
                        }
                    }

                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }
}