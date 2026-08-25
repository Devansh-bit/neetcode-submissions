

class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        self.max_diameter = 0

        def get_depth(node: Optional[TreeNode]) -> int:
            if not node:
                return 0
            
            # Recursively find the depth of left and right subtrees
            left_depth = get_depth(node.left)
            right_depth = get_depth(node.right)
            
            # The path length through the current node
            self.max_diameter = max(self.max_diameter, left_depth + right_depth)
            
            # Return height of the current subtree
            return 1 + max(left_depth, right_depth)

        get_depth(root)
        return self.max_diameter