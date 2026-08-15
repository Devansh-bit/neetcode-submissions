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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // First pass: count the nodes.
        let len = {
            let mut len = 0;
            let mut ptr = &head;
            while let Some(node) = ptr {
                len += 1;
                ptr = &node.next;
            }
            len
        };
        let idx = len - n as usize;
        
        let mut ptr = &mut head;
        for _ in 0..idx {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        *ptr = ptr.as_mut().unwrap().next.take();
        head
        
    }
}
