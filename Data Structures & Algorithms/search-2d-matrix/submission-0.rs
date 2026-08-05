    fn convert_to_idx(position: usize, num_cols: usize) -> (usize, usize) {
        (position / num_cols, position % num_cols)
    }

    fn convert_to_position(row: usize, col: usize, num_cols: usize) -> usize {
        row * num_cols + col
    }

impl Solution {


    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let num_rows = matrix.len();
        let num_cols = matrix[0].len();
        let mut left: i32 = 0;
        let mut right: i32 = (num_rows * num_cols) as i32 - 1;
        while left <= right {
            let mid_position = left + (right - left) / 2;
            let (row, col) = convert_to_idx(mid_position as usize, num_cols);
            let mid_val = matrix[row][col];
            if mid_val == target {
                return true;
            } else if mid_val < target {
                left = mid_position + 1;
            } else {
                if mid_position == 0 {break;}
                right = mid_position - 1;
            }
        }
        false
    }
}
