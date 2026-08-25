class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        inorder_map = {val: idx for idx, val in enumerate(inorder)}
        self.pre_idx = 0

        def helper(in_left: int, in_right: int) -> Optional[TreeNode]:
            if in_left > in_right:
                return None

            root_val = preorder[self.pre_idx]
            self.pre_idx += 1
            root = TreeNode(root_val)

            mid = inorder_map[root_val]

            # Elements from in_left to mid-1 belong to the left subtree
            root.left = helper(in_left, mid - 1)
            # Elements from mid+1 to in_right belong to the right subtree
            root.right = helper(mid + 1, in_right)

            return root

        return helper(0, len(inorder) - 1)