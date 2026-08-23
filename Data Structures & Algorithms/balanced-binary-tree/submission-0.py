class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def helper(node):
            if not node:
                return (0, True)
            
            left_height, left_balanced = helper(node.left)
            right_height, right_balanced = helper(node.right)
            
            current_balanced = abs(left_height - right_height) <= 1            
            overall_balanced = left_balanced and right_balanced and current_balanced
            
            return (1 + max(left_height, right_height), overall_balanced)
        
        return helper(root)[1]