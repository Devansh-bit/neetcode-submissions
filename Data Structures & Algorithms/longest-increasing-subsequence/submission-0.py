class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        dp = {}
        def dfs(i, last_index) -> int:
            if i == len(nums):
                return 0
            if (i, last_index) in dp:
                return dp[i, last_index]

            skip = dfs(i+1, last_index)

            pick = 0
            if last_index == -1 or nums[i] > nums[last_index]:
                pick = 1 + dfs(i + 1, i)
            
            dp[(i, last_index)] = max(skip, pick)
                
            return max(skip, pick)

        return dfs(0, -1)