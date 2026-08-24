class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        
        # Returns a tuple: (number_of_nodes_visited, answer_if_found)
        def inorder(node, count):
            # Base case: empty node means count hasn't changed, and we found nothing
            if not node:
                return count, None
            
            # 1. LEFT: Dive down, and get the updated count from the left subtree
            count, answer = inorder(node.left, count)
            
            # If the answer was found down there, immediately short-circuit and bubble it up
            if answer is not None:
                return count, answer
            
            # 2. NODE: Process the current node
            count += 1
            if count == k:
                return count, node.val  # We found it!
                
            # 3. RIGHT: Pass our updated count into the right subtree
            return inorder(node.right, count)
            
        # Start the recursion with a count of 0
        final_count, final_answer = inorder(root, 0)
        
        return final_answer