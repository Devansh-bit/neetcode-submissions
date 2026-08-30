class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        result = []
        def helper(index, subset:List[int]):
            if index == len(nums):
                result.append(subset)
                return
            s = subset.copy()
            s.append(nums[index])
            helper(index+1, s)
            helper(index+1, subset)
        helper(0, [])
        return list(result)






