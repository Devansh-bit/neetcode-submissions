impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let n = rows * cols;

        // Flatten into a single contiguous 1D slice in L1 cache
        let mut flat: Vec<i32> = Vec::with_capacity(n);
        for row in grid {
            flat.extend(row);
        }

        let mut max_area = 0;
        let mut stack = Vec::with_capacity(n);
        let ptr = flat.as_mut_ptr();

        for i in 0..n {
            // Safety: i is guaranteed within bounds [0..n)
            unsafe {
                if *ptr.add(i) == 1 {
                    *ptr.add(i) = 0;
                    let mut area = 0;
                    stack.push(i);

                    while let Some(curr) = stack.pop() {
                        area += 1;
                        let r = curr / cols;
                        let c = curr % cols;

                        // Down
                        if r + 1 < rows {
                            let next = curr + cols;
                            if *ptr.add(next) == 1 {
                                *ptr.add(next) = 0;
                                stack.push(next);
                            }
                        }
                        // Up
                        if r > 0 {
                            let next = curr - cols;
                            if *ptr.add(next) == 1 {
                                *ptr.add(next) = 0;
                                stack.push(next);
                            }
                        }
                        // Right
                        if c + 1 < cols {
                            let next = curr + 1;
                            if *ptr.add(next) == 1 {
                                *ptr.add(next) = 0;
                                stack.push(next);
                            }
                        }
                        // Left
                        if c > 0 {
                            let next = curr - 1;
                            if *ptr.add(next) == 1 {
                                *ptr.add(next) = 0;
                                stack.push(next);
                            }
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