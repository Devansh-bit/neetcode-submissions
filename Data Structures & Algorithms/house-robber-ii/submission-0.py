from typing import List

class Solution:
    def rob(self, nums: List[int]) -> int:
        # Edge case: only one house
        if len(nums) == 1:
            return nums[0]

        # Helper function: standard House Robber I for a linear street
        def rob_linear(houses: List[int]) -> int:
            prev2 = 0
            prev1 = 0
            for house in houses:
                current = max(prev1, house + prev2)
                prev2 = prev1
                prev1 = current
            return prev1

        # Case 1: Exclude the last house (nums[:-1])
        # Case 2: Exclude the first house (nums[1:])
        return max(rob_linear(nums[:-1]), rob_linear(nums[1:]))