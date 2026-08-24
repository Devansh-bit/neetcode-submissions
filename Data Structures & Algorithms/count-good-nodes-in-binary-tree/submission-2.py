# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        # running_max: max number seen in all ancestors in path
        if not root:
            return 0
        def helper(node, running_max):
            if not node:
                return 0
            is_good = 1 if node.val >= running_max else 0
            new_max = max(running_max, node.val)
            left_count = helper(node.left, new_max)
            right_count = helper(node.right, new_max)
            return is_good + left_count + right_count
        
        return helper(root, root.val)