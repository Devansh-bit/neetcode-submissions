import heapq
from typing import List

class MaxHeap:
    def __init__(self, array: List[int]):
        self._heap = [-item for item in array]
        heapq.heapify(self._heap)
    
    def __getitem__(self, index: int) -> int:
        return -self._heap[index]

    def __len__(self) -> int:
        return len(self._heap)
    
    def __bool__(self) -> bool:
        return bool(self._heap)

    def heappush(self, item: int):
        heapq.heappush(self._heap, -item)
    
    def heappop(self) -> int:
        return -heapq.heappop(self._heap)

class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        heap = MaxHeap(stones)

        while len(heap) > 1:
            y = heap.heappop()
            x = heap.heappop()
            if y != x:
                heap.heappush(y - x)

        return heap[0] if heap else 0