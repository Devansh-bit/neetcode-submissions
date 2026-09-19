class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        # Minimum cost to leave step (i - 2) and step (i - 1)
        min_cost_prev_prev = cost[0]
        min_cost_prev = cost[1]

        # Iterate from step 2 up to the top (len(cost))
        for i in range(2, len(cost)):
            # To leave step i, you must pay cost[i] plus the minimum of coming from (i-1) or (i-2)
            current_min_cost = cost[i] + min(min_cost_prev, min_cost_prev_prev)
            
            # Slide window forward
            min_cost_prev_prev = min_cost_prev
            min_cost_prev = current_min_cost

        # To reach the top floor, you can arrive from either the last or second-to-last step
        return min(min_cost_prev, min_cost_prev_prev)