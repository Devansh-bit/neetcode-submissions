class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        result: List[List[int]] = []

        def explore(index: int, subset: List[int]):
            if index == len(nums):
                result.append(subset.copy())
                return
            
            subset.append(nums[index])
            explore(index+1, subset)

            num = subset.pop()
            while index+1 < len(nums) and nums[index+1] == num:
                index += 1
            explore(index+1, subset)

        explore(0, [])
        return result