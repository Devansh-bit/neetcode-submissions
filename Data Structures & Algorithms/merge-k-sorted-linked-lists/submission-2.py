import heapq

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        heap = []
        
        # Push initial heads: (val, index, node)
        # The index breaks ties when two nodes have identical values
        for i, node in enumerate(lists):
            if node:
                heapq.heappush(heap, (node.val, i, node))
        
        head = ListNode()
        tail = head
        
        while heap:
            val, i, node = heapq.heappop(heap)
            tail.next = node
            tail = tail.next
            
            if node.next:
                heapq.heappush(heap, (node.next.val, i, node.next))
                
        return head.next