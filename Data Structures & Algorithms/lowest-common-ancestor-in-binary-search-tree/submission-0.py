# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:
        if not p or not q:
            return root
        if p.val > q.val:
            (p, q) = (q, p) # p is smaller
        while True:
            if not root:
                return None
            if p.val == root.val or q.val == root.val:
                return root
            if p.val < root.val < q.val:
                return root
            elif root.val < p.val:
                root = root.right
            else: # q.val < root.val
                root = root.left

