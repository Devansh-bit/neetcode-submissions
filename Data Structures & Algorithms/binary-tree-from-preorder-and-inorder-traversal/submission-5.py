class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        if not preorder:
            return None

        root = TreeNode(preorder[0])
        stack = [root]
        in_idx = 0

        for val in preorder[1:]:
            node = TreeNode(val)
            parent = stack[-1]

            # If stack top does not match inorder[in_idx], it is a left child
            if parent.val != inorder[in_idx]:
                parent.left = node
            else:
                # Pop back up to find the parent whose right child this belongs to
                while stack and stack[-1].val == inorder[in_idx]:
                    parent = stack.pop()
                    in_idx += 1
                parent.right = node

            stack.append(node)

        return root