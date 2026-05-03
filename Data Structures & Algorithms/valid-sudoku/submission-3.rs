impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut blocks = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {continue;}
                let bit = 1u16 << (ch as u8 - b'1');
                let b = (r/3)*3 + c/3;
                if (rows[r] | cols[c] | blocks[b]) & bit != 0 {
                    return false;
                }
                rows[r] |= bit;
                cols[c] |= bit;
                blocks[b] |= bit;
            }
        }
        true
    }
}
