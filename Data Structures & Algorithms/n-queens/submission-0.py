class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        result: List[List[str]] = []
        cols_filled = []

        def solution():
            board = []
            for col in cols_filled:
                row_str = "." * col + "Q" + "." * (n - col - 1)
                board.append(row_str)
            result.append(board)

        def explore(row):
            if row == n:
                solution()
                return
            
            def check_valid(row, col):
                for filled_row, filled_col in enumerate(cols_filled):
                    if filled_col == col or abs(filled_row - row) == abs(filled_col - col):
                        return False
                return True


            for col in range(0, n):
                if not check_valid(row, col):
                    continue
                
                cols_filled.append(col)
                explore(row+1)
                cols_filled.pop()
        
        explore(0)
        return result
            