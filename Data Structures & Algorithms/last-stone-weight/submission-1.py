import heapq

class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        heap = [-x for x in stones]
        heapq.heapify(heap)
        while len(heap) > 1:
            y = -heapq.heappop(heap)
            x = -heapq.heappop(heap)
            if y > x:
                heapq.heappush(heap, x-y)
        if len(heap) == 1:
            return -heapq.heappop(heap)
        else:
            return 0