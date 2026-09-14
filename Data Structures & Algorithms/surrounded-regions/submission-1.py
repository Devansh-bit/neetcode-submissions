from typing import List

class Solution:
    def solve(self, board: List[List[str]]) -> None:
        if not board or not board[0]:
            return
        rows, cols = len(board), len(board[0])

        # Seed the stack with every "O" on the border.
        stack = [(r, c)
                 for r in range(rows) for c in range(cols)
                 if (r in (0, rows - 1) or c in (0, cols - 1)) and board[r][c] == "O"]

        # Flood-fill inward, marking border-connected "O"s as safe ("#").
        while stack:
            r, c = stack.pop()
            if 0 <= r < rows and 0 <= c < cols and board[r][c] == "O":
                board[r][c] = "#"
                stack.extend([(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)])

        # Anything still "O" is surrounded; restore the safe ones.
        for r in range(rows):
            for c in range(cols):
                if board[r][c] == "O":
                    board[r][c] = "X"
                elif board[r][c] == "#":
                    board[r][c] = "O"