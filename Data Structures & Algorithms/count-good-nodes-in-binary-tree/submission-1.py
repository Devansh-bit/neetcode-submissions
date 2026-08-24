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
        self.res = 0
        def helper(node, running_max):
            if not node:
                return
            if node.val >= running_max:
                self.res += 1
            running_max = max(node.val, running_max)
            if node.left:
                helper(node.left, running_max)
            if node.right:
                helper(node.right, running_max)
        helper(root, root.val)
        return self.res