class Solution:
    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        result: List[List[int]] = []
        def explore(index, subset, current_sum):
            if current_sum == target:
                result.append(subset.copy())
                return
            if index == len(nums) or current_sum > target:
                return
            
            current_num = nums[index]
            #Explore
            subset.append(current_num)
            current_sum += current_num
            explore(index, subset, current_sum)

            # Backtrack
            current_sum -= current_num
            subset.pop()
            explore(index+1, subset, current_sum)
        
        explore(0, [], 0)
        return result
            