class Solution:
    def rob(self, nums: List[int]) -> int:
        max_money_prev_prev = 0
        max_money_prev = 0

        for cost in nums:
            current_best = max(max_money_prev, cost + max_money_prev_prev)
            max_money_prev_prev = max_money_prev
            max_money_prev = current_best

        return current_best