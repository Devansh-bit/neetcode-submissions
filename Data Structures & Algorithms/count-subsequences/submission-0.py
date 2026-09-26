class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        memo = {}
        def dfs(si, ti):
            if ti == len(t):
                return 1
            if si == len(s):
                return 0
            if (si, ti) in memo:
                return memo[si, ti]
            
            take = 0
            if s[si] == t[ti]:
                # take
                take = dfs(si+1, ti+1)
            skip = dfs(si+1, ti)
            memo[si, ti] = skip + take
            return skip + take
        return dfs(0, 0)