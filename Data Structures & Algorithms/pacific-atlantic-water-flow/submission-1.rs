use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty() || heights[0].is_empty() {
            return vec![];
        }
        let rows = heights.len();
        let cols = heights[0].len();

        let mut pacific = HashSet::new();
        let mut atlantic = HashSet::new();

        // Nested fn: allowed, because it captures nothing —
        // everything it needs is passed in explicitly.
        fn dfs(
            heights: &Vec<Vec<i32>>,
            r: usize,
            c: usize,
            reachable: &mut HashSet<(usize, usize)>,
            rows: usize,
            cols: usize,
        ) {
            reachable.insert((r, c));

            let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for (dr, dc) in dirs {
                // Compute the neighbor in i32 so -1 can't underflow a usize.
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                // Bounds check (no chained comparisons).
                if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);

                // Already recorded? Skip — this is what stops infinite loops.
                if reachable.contains(&(nr, nc)) {
                    continue;
                }

                // Reversed flow: we can climb to the neighbor only if it's
                // at least as high as where we are now.
                if heights[nr][nc] < heights[r][c] {
                    continue;
                }

                dfs(heights, nr, nc, reachable, rows, cols);
            }
        }

        // Pacific: top row + left column.
        for c in 0..cols {
            dfs(&heights, 0, c, &mut pacific, rows, cols);
        }
        for r in 0..rows {
            dfs(&heights, r, 0, &mut pacific, rows, cols);
        }

        // Atlantic: bottom row + right column.
        for c in 0..cols {
            dfs(&heights, rows - 1, c, &mut atlantic, rows, cols);
        }
        for r in 0..rows {
            dfs(&heights, r, cols - 1, &mut atlantic, rows, cols);
        }

        // Cells reachable from both oceans.
        pacific
            .intersection(&atlantic)
            .map(|&(r, c)| vec![r as i32, c as i32])
            .collect()
    }
}