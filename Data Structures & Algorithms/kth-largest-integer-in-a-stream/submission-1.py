import heapq
from typing import List

class FixedSizeMinHeap:
    def __init__(self, max_size: int):
        self.max_size = max_size
        self.heap = []

    def push(self, item):
        """Adds an item, maintaining the maximum capacity constraint."""
        if len(self.heap) < self.max_size:
            heapq.heappush(self.heap, item)
        else:
            heapq.heappushpop(self.heap, item)

    def peek_min(self):
        """Returns the smallest item currently in the heap without removing it."""
        return self.heap[0] if self.heap else None

    def pop_min(self):
        """Removes and returns the smallest item from the heap."""
        return heapq.heappop(self.heap) if self.heap else None

    def get_all_sorted(self):
        """Returns all elements sorted from smallest to largest."""
        return sorted(self.heap)

    def __len__(self):
        return len(self.heap)

class KthLargest:

    def __init__(self, k: int, nums: List[int]):
        self.heap = FixedSizeMinHeap(k)
        for num in nums:
            self.heap.push(num)

    def add(self, val: int) -> int:
        self.heap.push(val)
        return self.heap.peek_min()