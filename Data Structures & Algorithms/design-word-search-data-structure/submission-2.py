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