class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        memo = {}
        def dfs(i, curr_sum) -> int:
            if i == len(nums):
                return 1 if curr_sum == target else 0
            if (i, curr_sum) in memo:
                return memo[i, curr_sum]
            
            # add
            add_ways = dfs(i+1, curr_sum+nums[i])
            sub_ways = dfs(i+1, curr_sum-nums[i])
            memo[i, curr_sum] = add_ways + sub_ways
            return memo[i, curr_sum]
        return dfs(0, 0)

