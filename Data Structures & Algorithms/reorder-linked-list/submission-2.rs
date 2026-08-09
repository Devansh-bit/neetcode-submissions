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
    fn reverse(head: &mut Option<Box<ListNode>>) {
        let mut current = head.take();
        let mut prev = None;

        while let Some(mut node) = current {
            current = node.next;
            node.next = prev;
            prev = Some(node);
        }

        *head = prev;
    }

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut len: usize = 0;
        let mut current: &_ = head;
        while let Some(node) = current.as_ref() {
            len += 1;
            current = &node.next;
        }

        let mut split_len = (len+1)/2;
        let mut left_head = head.take();
        let mut right_half = &mut left_head;
        while split_len > 0 {
            split_len -= 1;
            right_half = &mut right_half.as_mut().unwrap().next;
        }
        let mut right_head = right_half.take();
        Self::reverse(&mut right_head);

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy;
        while right_head.is_some() {
            let mut left = left_head.take().unwrap();
            let mut right = right_head.take().unwrap();
            left_head = left.next.take();
            right_head = right.next.take();

            tail.as_mut().unwrap().next = Some(left);
            tail = &mut tail.as_mut().unwrap().next;
            tail.as_mut().unwrap().next = Some(right);
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = left_head;
        *head = dummy.unwrap().next.take();

    }
}