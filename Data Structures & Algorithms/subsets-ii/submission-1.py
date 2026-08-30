class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        result = [[]]
        prev_end = 0

        for i in range(len(nums)):
            # If duplicate, only extend subsets added in the last round
            start = prev_end if (i > 0 and nums[i] == nums[i - 1]) else 0
            prev_end = len(result)
            
            # Extend existing subsets with nums[i]
            for j in range(start, prev_end):
                result.append(result[j] + [nums[i]])

        return result