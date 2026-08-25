class Codec:
    
    # Encodes a tree to a single string.
    def serialize(self, root: Optional[TreeNode]) -> str:
        out = []
        def dfs(node):
            if not node:
                out.append("#")  # Use a clear marker string
                return
            out.append(str(node.val))
            dfs(node.left)
            dfs(node.right)

        dfs(root)
        return ",".join(out)  # Comma is safer if values contain symbols
        
    # Decodes your encoded data to tree.
    def deserialize(self, data: str) -> Optional[TreeNode]:
        # Convert string back to a list of tokens
        tokens = data.split(",")
        # Use an iterator to track our position across recursive calls
        vals = iter(tokens)
        
        def build():
            val = next(vals)
            if val == "#":
                return None
            
            # Create the node and reconstruct subtrees in pre-order
            node = TreeNode(int(val))
            node.left = build()
            node.right = build()
            return node
            
        return build()
