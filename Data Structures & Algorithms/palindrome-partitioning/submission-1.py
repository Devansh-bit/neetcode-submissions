from typing import List

class Solution:
    def partition(self, s: str) -> List[List[str]]:
        result: List[List[str]] = []
        path: List[str] = []
        def is_palindrome(sub: str) -> bool:
            return sub == sub[::-1]

        def explore(start):
            if start == len(s):
                result.append(list(path))
                return
            for end in range(start, len(s)):
                substring = s[start:end+1]
                if is_palindrome(substring):
                    path.append(substring)
                    explore(end+1)
                    path.pop()
        explore(0)
        return result