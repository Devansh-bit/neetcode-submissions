class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        trie = WordDictionary()
        for word in words:
            trie.addWord(word)
            
        rows = len(board)
        cols = len(board[0])
        res = []
        
        def dfs(node, row, col, path):
            # 1. If we reached a word ending, collect it and unmark to avoid duplicates
            if "$" in node:
                res.append(path)
                del node["$"]
            
            char = board[row][col]
            board[row][col] = "#"
            
            # 2. Explore neighbors
            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                nr, nc = row + dr, col + dc
                if 0 <= nr < rows and 0 <= nc < cols and board[nr][nc] != "#":
                    next_char = board[nr][nc]
                    if next_char in node:
                        dfs(node[next_char], nr, nc, path + next_char)
            
            # 3. Backtrack
            board[row][col] = char

        # 4. Trigger DFS for any cell that matches a top-level character in trie.root
        for row in range(rows):
            for col in range(cols):
                char = board[row][col]
                if char in trie.root:
                    dfs(trie.root[char], row, col, char)
                    
        return res

class WordDictionary:
    def __init__(self):
        self.root = {}

    def addWord(self, word: str) -> None:
        node = self.root
        for char in word:
            if char not in node:
                node[char] = {}
            node = node[char]
        node["$"] = True

    def search(self, word: str) -> bool:
        def dfs(node: dict, i: int) -> bool:
            if i == len(word):
                return "$" in node
            
            char = word[i]
            if char == ".":
                for key, child in node.items():
                    if key != "$" and dfs(child, i + 1):
                        return True
                return False
            
            if char not in node:
                return False
            return dfs(node[char], i + 1)

        return dfs(self.root, 0)