class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        if len(nums) == 0:
            return 0
        current_max = 1
        current_min = 1
        res = nums[0]
        
        for n in nums:
            if n < 0:
                current_max, current_min = current_min, current_max

            current_max = max(n, n * current_max)
            current_min = min(n, n * current_min)

            res = max(res, current_max)

        return res

        
