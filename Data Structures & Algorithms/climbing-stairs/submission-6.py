class Solution:
    def climbStairs(self, n: int) -> int:
        prev = 1
        prev_prev = 1
        for i in range(2, n+1):
            prev, prev_prev = prev + prev_prev, prev

        return prev

