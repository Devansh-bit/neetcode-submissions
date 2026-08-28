class Node:
    def __init__(self, val: str = "", word_end: bool = False):
        self.val = val
        self.word_end = word_end
        self.children = {} 


class PrefixTree:
    def __init__(self):
        self.root = Node("")

    def insert(self, word: str) -> None:
        curr = self.root
        for char in word:
            if char not in curr.children:
                curr.children[char] = Node(char)
            curr = curr.children[char]
        curr.word_end = True

    def search(self, word: str) -> bool:
        curr = self.root
        for char in word:
            if char not in curr.children:
                return False
            curr = curr.children[char]
        return curr.word_end

    def startsWith(self, prefix: str) -> bool:
        curr = self.root
        for char in prefix:
            if char not in curr.children:
                return False
            curr = curr.children[char]
        return True