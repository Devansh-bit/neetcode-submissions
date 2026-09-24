class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        memo = {}
        
        def dfs(i, j):
            # Base Case: If either pointer falls off the edge, 
            # one string is empty. The common subsequence is 0.
            if i == len(text1) or j == len(text2):
                return 0
                
            if (i, j) in memo:
                return memo[(i, j)]
                
            # Reality 1: Match
            if text1[i] == text2[j]:
                memo[(i, j)] = 1 + dfs(i + 1, j + 1)
                
            # Reality 2: Mismatch
            else:
                memo[(i, j)] = max(dfs(i + 1, j), dfs(i, j + 1))
                
            return memo[(i, j)]
            
        return dfs(0, 0)