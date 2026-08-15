# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        ptr1 = l1
        ptr2 = l2
        head = ListNode()
        tail = head
        carry = False
        while ptr1 is not None or ptr2 is not None:
            digit = 1 if carry else 0
            if ptr1:
                digit = digit + ptr1.val
                ptr1 = ptr1.next
            if ptr2:
                digit = digit + ptr2.val
                ptr2 = ptr2.next
            carry = digit >= 10
            digit = digit % 10

            tail.next = ListNode(digit)
            tail = tail.next
        if carry:
            tail.next = ListNode(1)
        return head.next


