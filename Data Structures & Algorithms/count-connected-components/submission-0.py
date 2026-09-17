class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        arr = [i for i in range(n)]
        size = [1] * n
        
        # Start by assuming every node is its own separate component
        components = n 
        
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
            if union(a, b):
                # We successfully merged two separate sets, so the total count drops by 1
                components -= 1

        return components