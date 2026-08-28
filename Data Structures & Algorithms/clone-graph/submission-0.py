class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if not node:
            return None

        clones = {}

        def dfs(curr):
            nonlocal clones
            if curr in clones:
                return clones[curr]

            copy = Node(curr.val)
            clones[curr] = copy
            copy.neighbors = [dfs(neighbor) for neighbor in curr.neighbors]
            return copy

        return dfs(node)