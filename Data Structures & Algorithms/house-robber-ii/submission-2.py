class Solution:
    def rob(self, nums: List[int]) -> int:
        def work(take_0):
            prev = 0
            prev_prev = 0
            if take_0:
                prev = nums[0]
                end = len(nums) - 1
                start = 1
            else:
                start = 1
                end = len(nums)
            for i in range(start, end):
                prev, prev_prev = max(prev, prev_prev + nums[i]), prev
            return prev
        return max(work(True), work(False))
        