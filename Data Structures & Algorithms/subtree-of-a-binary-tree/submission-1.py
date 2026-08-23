class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def isSameTree(p, q):
            if not p and not q:
                return True
            if not p or not q or p.val != q.val:
                return False
            return isSameTree(p.left, q.left) and isSameTree(p.right, q.right)
        
        if not subRoot:
            return True 
        if not root:
            return False 
            
        # 1. Does the tree starting at this exact node match subRoot?
        if isSameTree(root, subRoot):
            return True
            
        # 2. If not, recursively check if it's a subtree hiding in the left OR right branch
        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)