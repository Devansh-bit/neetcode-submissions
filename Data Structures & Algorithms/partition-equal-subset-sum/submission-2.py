class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total_sum = sum(nums)
        
        # If the sum is odd, it's impossible to halve it
        if total_sum % 2 != 0:
            return False
            
        target = total_sum // 2
        dp = {0} # Set containing all reachable sums
        
        for num in nums:
            next_dp = set()
            for current_sum in dp:
                new_sum = current_sum + num
                
                # If we hit the target, we are done immediately
                if new_sum == target:
                    return True
                    
                # Only keep sums that do not overshoot the target
                if new_sum < target:
                    next_dp.add(new_sum)
                    
            # Merge the newly reachable sums with the previously reachable sums
            dp.update(next_dp)
            
        return False