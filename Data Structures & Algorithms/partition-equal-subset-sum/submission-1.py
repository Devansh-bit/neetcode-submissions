class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        s = sum(nums)
        if s % 2 != 0:
            return False
        memo = {}
        target = s / 2
        def dfs(i, cur_sum):
            if cur_sum == target:
                return True
            if i >= len(nums) or cur_sum > target:
                return False
            if (i, cur_sum) in memo:
                return memo[(i, cur_sum)]

            pick = dfs(i + 1, cur_sum + nums[i])
            skip = dfs(i + 1, cur_sum)

            memo[(i, cur_sum)] = pick or skip
            return pick or skip
        return dfs(0, 0)