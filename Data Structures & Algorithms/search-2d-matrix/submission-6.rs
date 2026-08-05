impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        // 1. Use partition_point on the matrix slice to find the first row 
        //    whose LAST element is >= target
        let row_idx = matrix.partition_point(|row| row[row.len() - 1] < target);

        // 2. If row_idx is in bounds, use binary_search directly on that row's slice
        if row_idx < matrix.len() {
            matrix[row_idx].binary_search(&target).is_ok()
        } else {
            false
        }
    }
}