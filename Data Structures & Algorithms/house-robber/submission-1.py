class Solution:
    def rob(self, nums: List[int]) -> int:
        # money(i) = max(money(i-1), money(i-2) + money(i))
        prev = 0
        prev_prev = 0
        for i in range(len(nums)):
            prev, prev_prev = max(prev, prev_prev+nums[i]), prev
        return prev
