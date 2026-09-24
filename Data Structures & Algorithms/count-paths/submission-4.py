import math

class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        # Total number of moves is (m - 1) + (n - 1)
        total_moves = m + n - 2
        
        # We need to choose (m - 1) slots to go down
        down_moves = m - 1
        
        return math.comb(total_moves, down_moves)