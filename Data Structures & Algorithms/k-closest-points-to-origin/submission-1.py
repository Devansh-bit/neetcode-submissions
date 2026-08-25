class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        heap = []
        for point in points:
            distance = point[0]**2 + point[1]**2
            if len(heap) < k:
                heapq.heappush(heap, (-distance, point))
            else:
                heapq.heappushpop(heap, (-distance, point))
        return [item[1] for item in heap]