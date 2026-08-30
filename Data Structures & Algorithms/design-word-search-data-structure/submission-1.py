class Node:
    def __init__(self, val):
        self.val = val
        self.is_end = False
        self.children = {}

class WordDictionary:
    def __init__(self):
        self.root = Node(None)

    def addWord(self, word: str) -> None:
        current = self.root
        for char in word:
            if char not in current.children:
                current.children[char] = Node(char)
            current = current.children[char]
        current.is_end=True

    def search(self, word: str) -> bool:
        def dfs(node, i) -> bool:
            if len(word) == i:
                return node.is_end
            char = word[i]
            if char == ".":
                for child in node.children.values():
                    if dfs(child, i+1):
                        return True
                return False
            if char not in node.children:
                return False
            return dfs(node.children[char], i+1)
        return dfs(self.root, 0)
        