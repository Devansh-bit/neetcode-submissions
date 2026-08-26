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
    def findKthLargest(self, nums: List[int], k: int) -> int:
        heap = MaxHeap(nums)
        for i in range(k-1):
            heap.heappop()
        return heap[0]