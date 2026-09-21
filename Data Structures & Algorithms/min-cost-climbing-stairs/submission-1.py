class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        cost_prev = 0
        cost_prev_prev = 0
        n = len(cost)
        for i in range(2, n+1):
            cost_prev, cost_prev_prev = min(cost_prev + cost[i-1], cost_prev_prev + cost[i-2]), cost_prev
        return cost_prev
