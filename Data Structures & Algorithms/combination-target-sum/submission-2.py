class Solution:
    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        nums.sort()  # Enables early stopping
        result: List[List[int]] = []
        subset: List[int] = []

        def explore(index: int, current_sum: int):
            if current_sum == target:
                result.append(subset.copy())
                return
            if index == len(nums) or current_sum + nums[index] > target:
                return  # Prunes entire branch early since remaining numbers are even larger

            current_num = nums[index]

            # Choice 1: Include and stay at index
            subset.append(current_num)
            explore(index, current_sum + current_num)

            # Backtrack
            subset.pop()

            # Choice 2: Exclude and move forward
            explore(index + 1, current_sum)

        explore(0, 0)
        return result