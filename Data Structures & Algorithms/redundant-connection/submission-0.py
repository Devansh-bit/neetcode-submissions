class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        n = len(edges) # 1. Define n
        
        # 2. Make arrays size n + 1 to handle 1-based indexing (nodes 1 to n)
        arr = [i for i in range(n + 1)] 
        size = [1] * (n + 1)
        
        def find(i):
            if arr[i] != i:
                arr[i] = find(arr[i])
            return arr[i]

        def union(a, b):
            root_a = find(a)
            root_b = find(b)
            
            if root_a == root_b:
                return False 
                
            if size[root_a] < size[root_b]:
                arr[root_a] = root_b
                size[root_b] += size[root_a]
            else:
                arr[root_b] = root_a
                size[root_a] += size[root_b]
                
            return True 

        for a, b in edges:
            if not union(a, b):
                return [a, b]
                
        return []