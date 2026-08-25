# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        if not root.left and not root.right:
            return root.val
        self.max_sum = float('-inf')
        def dfs(node) -> int:
            if not node:
                return 0
            left_sum = dfs(node.left)
            right_sum = dfs(node.right)
            max_path_sum = max(left_sum + node.val, right_sum + node.val, 0)
            current_path_sum = left_sum + right_sum + node.val
            self.max_sum = max(current_path_sum, self.max_sum)
            return max_path_sum
        dfs(root)
        return self.max_sum