class Solution:
    def climbStairs(self, n: int) -> int:
        if n <= 2:
            return n

        # prev1 represents ways(i - 1), prev2 represents ways(i - 2)
        prev2, prev1 = 1, 2

        for _ in range(3, n + 1):
            prev2, prev1 = prev1, prev1 + prev2

        return prev1  