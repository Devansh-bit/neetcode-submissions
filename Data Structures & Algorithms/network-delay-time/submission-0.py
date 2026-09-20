class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        adj = {i: [] for i in range(1, n+1)}
        for (ui, vi, ti) in times:
            adj[ui].append((ti, vi))
        
        heap = [(0, k)]
        visited = set()
        max_time = 0
        while heap:
            current_time, current_node = heapq.heappop(heap)
            if current_node in visited:
                continue
            visited.add(current_node)
            max_time = current_time
            if len(visited) == n:
                return max_time
            for edge_time, destination in adj[current_node]:
                heapq.heappush(heap, (current_time + edge_time, destination))

        return max_time if len(visited) == n else -1