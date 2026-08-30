class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        result = []
        nums.sort()
        def explore(index, subset):
            if index == len(nums):
                result.append(subset.copy())
                return
            
            subset.append(nums[index])
            explore(index+1, subset)
            popped = subset.pop()

            while index+1 < len(nums) and nums[index+1] == popped:
                index += 1
            
            explore(index+1, subset)

        explore(0, [])
        return result        
