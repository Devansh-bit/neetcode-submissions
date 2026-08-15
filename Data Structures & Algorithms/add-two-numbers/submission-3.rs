// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = false;
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy;
        while l1.is_some() || l2.is_some() || carry {
            let mut digit = if carry {1} else {0};
            if let Some(node) = l1.take() {
                digit += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2.take() {
                digit += node.val;
                l2 = node.next;
            }
            carry = digit >= 10;
            digit = digit % 10;

            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(digit)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        dummy.unwrap().next
    }
}
