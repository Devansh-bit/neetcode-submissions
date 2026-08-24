class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        # Base Cases: 
        # 1. We hit a dead end (None)
        # 2. We found either p or q! 
        if not root or root == p or root == q:
            return root
        
        # Post-Order: Search the left and right subtrees first
        left = self.lowestCommonAncestor(root.left, p, q)
        right = self.lowestCommonAncestor(root.right, p, q)
        
        # Evaluate what the children reported back:
        
        # Case 1: Both children found a target. 
        # This node is the split point, so it MUST be the LCA!
        if left and right:
            return root
            
        # Case 2: Only one child found a target.
        # This means BOTH targets are on that side, so just pass the found node up.
        # (Or it means we found one target, and the other isn't in this subtree at all).
        if left:
            return left
        if right:
            return right
            
        # Case 3: Neither child found anything.
        return None