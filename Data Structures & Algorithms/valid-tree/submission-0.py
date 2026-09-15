class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:

        graph = {}
        graph = {i: [] for i in range(n)}
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)

        print(graph)
        path = set()
        visited = set()
        def dfs(node, from_node):
            if node in path:
                return False
            if node in visited:
                return True
            
            path.add(node)
            for next_node in graph[node]:
                if next_node == from_node:
                    continue
                if not dfs(next_node, node):
                    return False
            path.remove(node)
            visited.add(node)
            return True
        
        if not dfs(0, -1):
            return False          # cycle found
        return len(visited) == n  # everything reachable?
            