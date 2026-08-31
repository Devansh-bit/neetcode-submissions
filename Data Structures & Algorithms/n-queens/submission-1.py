class Solution:
    def solveNQueens(self, n: int) -> list[list[str]]:
        result = []
        cols = set()
        pos_diag = set()  # (r + c)
        neg_diag = set()  # (r - c)
        
        cols_filled = []

        def explore(r: int):
            if r == n:
                board = ["." * c + "Q" + "." * (n - c - 1) for c in cols_filled]
                result.append(board)
                return

            for c in range(n):
                if c in cols or (r + c) in pos_diag or (r - c) in neg_diag:
                    continue

                # Add state
                cols.add(c)
                pos_diag.add(r + c)
                neg_diag.add(r - c)
                cols_filled.append(c)

                explore(r + 1)

                # Backtrack state
                cols.remove(c)
                pos_diag.remove(r + c)
                neg_diag.remove(r - c)
                cols_filled.pop()

        explore(0)
        return result