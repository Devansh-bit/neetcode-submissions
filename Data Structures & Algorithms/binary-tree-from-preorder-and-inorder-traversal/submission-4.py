class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        pre_iter = iter(preorder)
        in_iter = iter(inorder)
        in_curr = next(in_iter)

        def build(stop=None) -> Optional[TreeNode]:
            nonlocal in_curr
            if in_curr == stop:
                return None

            val = next(pre_iter, None)
            if val is None:
                return None

            root = TreeNode(val)
            root.left = build(val)
            
            # Matched root in inorder, advance inorder pointer
            in_curr = next(in_iter, None)
            root.right = build(stop)
            
            return root

        return build()