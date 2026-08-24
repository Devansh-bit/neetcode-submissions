# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        self.result = []
        def add_level_order(level_nodes):
            if not level_nodes:
                return
            res = []
            next_level = []
            for node in level_nodes:
                if not node:
                    continue
                if node.left:
                    next_level.append(node.left)
                if node.right:
                    next_level.append(node.right)
                res.append(node.val)
            if res:
                self.result.append(res)
            add_level_order(next_level)
        add_level_order([root])
        return self.result
            
            
