class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        
        def validate(node, low, high):
            # Base case: empty nodes are always valid
            if not node:
                return True
            
            # The current node MUST be strictly inside the boundaries
            if not (low < node.val < high):
                return False
            
            # Go left: The max allowed value becomes the current node's value
            left_is_valid = validate(node.left, low, node.val)
            
            # Go right: The min allowed value becomes the current node's value
            right_is_valid = validate(node.right, node.val, high)
            
            return left_is_valid and right_is_valid
            
        # The root can be any number, so it starts with infinite boundaries
        return validate(root, float('-inf'), float('inf'))