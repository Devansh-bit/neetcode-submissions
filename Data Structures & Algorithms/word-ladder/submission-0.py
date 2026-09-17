from collections import deque
from typing import List

class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        # 1. Convert to set for O(1) lookups
        wordSet = set(wordList)
        
        # Quick exit: if the target isn't even in the list, it's impossible
        if endWord not in wordSet:
            return 0
            
        # 2. Initialize the queue with the starting word and step count = 1
        queue = deque([(beginWord, 1)])
        
        while queue:
            word, steps = queue.popleft()
            
            # If we hit our target, we are done!
            if word == endWord:
                return steps
                
            # 3. Generate all possible 1-letter transformations
            for i in range(len(word)):
                for c in 'abcdefghijklmnopqrstuvwxyz':
                    # Slice the string to replace the character at index i
                    next_word = word[:i] + c + word[i+1:]
                    
                    # 4. Check if this valid transformation exists in our dictionary
                    if next_word in wordSet:
                        # Remove it IMMEDIATELY so we never visit it again (prevents cycles)
                        wordSet.remove(next_word)
                        queue.append((next_word, steps + 1))
                        
        # If the queue empties and we never found the endWord
        return 0