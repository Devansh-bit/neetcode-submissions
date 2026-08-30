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
        def dfs(node, word) -> bool:
            if len(word) == 0:
                return node.is_end
            if word[0] == ".":
                res = []
                for (_, child) in node.children.items():
                    res.append(dfs(child, word[1:]))
                return any(res)
            else:
                if word[0] not in node.children:
                    return False
                return dfs(node.children[word[0]], word[1:])
        return dfs(self.root, word)
        