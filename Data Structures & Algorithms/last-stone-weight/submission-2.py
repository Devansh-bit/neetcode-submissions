import heapq
from typing import List

class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        heap = [-x for x in stones]
        heapq.heapify(heap)

        while len(heap) > 1:
            y = heapq.heappop(heap)
            x = heapq.heappop(heap)
            if y != x:
                # y is more negative than x, so y - x is negative
                heapq.heappush(heap, y - x)

        return -heap[0] if heap else 0