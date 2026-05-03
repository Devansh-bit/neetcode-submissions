impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // check rows
        let mut transposed: Vec<Vec<char>> = vec![vec!['.'; 9]; 9];

        let mut row_sets = vec![HashSet::new(); 9];
        let mut block_sets = vec![HashSet::new(); 9];

        for r in 0..9 {
            let mut col_set = HashSet::new();
            for c in 0..9 {
                let current = board[r][c];
                if current == '.' { continue; }

                if !col_set.insert(current) || !row_sets[c].insert(current) {
                    return false;
                }     

                if !block_sets[(r / (3 as usize))*3 + c / (3 as usize)].insert(current) {
                    return false;
                }
            }
        }
        true
    }
}
