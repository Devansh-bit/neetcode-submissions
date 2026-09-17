class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        # Condition 1: A valid tree with n nodes MUST have exactly n - 1 edges.
        # If it doesn't, it either has a cycle or is disconnected.
        if len(edges) != n - 1:
            return False

        arr = [i for i in range(n)]
        size = [1] * n
        
        def find(i):
            if arr[i] != i:
                arr[i] = find(arr[i])
            return arr[i]

        def union(a, b):
            root_a = find(a)
            root_b = find(b)
            
            # If they share the same root, adding this edge creates a cycle
            if root_a == root_b:
                return False 
                
            if size[root_a] < size[root_b]:
                arr[root_a] = root_b
                size[root_b] += size[root_a]
            else:
                arr[root_b] = root_a
                size[root_a] += size[root_b]
                
            return True # Added this to indicate a successful merge

        # Condition 2: Check for cycles
        for a, b in edges:
            if not union(a, b):
                # If union returns False, we found a cycle
                return False

        return True