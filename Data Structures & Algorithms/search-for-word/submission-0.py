class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        def dfs(index, r, c):
            if index == len(word):
                return True
            # Explore
            if not board[r][c] == word[index]:
                return False
            for dr, dc in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                if not(0 <= r+dr < len(board) and 0 <= c+dc < len(board[0])):
                    continue
                if board[r+dr][c+dc] != "$":
                    char = board[r][c]
                    board[r][c] = "$"
                    dfs(index+1, r+dr, c+dc)
                    board[r][c] = char # backtracking

        for r in range(len(board)):
            for c in range(len(board[0])):
                if word[0] == board[r][c]:
                    if dfs(0, r, c):
                        return True
        return False

class Solution:
    def exist(self, board: list[list[str]], word: str) -> bool:
        rows, cols = len(board), len(board[0])

        def dfs(index: int, r: int, c: int) -> bool:
            # If current character doesn't match, abort
            if board[r][c] != word[index]:
                return False
            
            # If we matched the last character of the word, we found it
            if index == len(word) - 1:
                return True

            # Mark cell as visited
            temp = board[r][c]
            board[r][c] = "#"

            # Explore 4 directions
            for dr, dc in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                nr, nc = r + dr, c + dc
                if 0 <= nr < rows and 0 <= nc < cols and board[nr][nc] != "#":
                    if dfs(index + 1, nr, nc):
                        board[r][c] = temp  # Clean up before returning
                        return True

            # Backtrack
            board[r][c] = temp
            return False

        for r in range(rows):
            for c in range(cols):
                if board[r][c] == word[0]:
                    if dfs(0, r, c):
                        return True

        return False