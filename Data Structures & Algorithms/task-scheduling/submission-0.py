from collections import Counter
from typing import List

class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        frequency = Counter(tasks)
        max_freq = max(frequency.values())
        max_count = len([item for item, count in frequency.items() if count == max_freq])
        
        # Calculate the required grid size based on the most frequent tasks
        grid_size = (max_freq - 1) * (n + 1) + max_count
        
        # If we have enough unique tasks to overflow the grid without idle time,
        # the answer is just the total number of tasks.
        return max(len(tasks), grid_size)