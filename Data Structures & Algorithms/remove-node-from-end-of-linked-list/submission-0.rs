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
        let mut len: usize = 0;
        let mut ptr = &head;
        while let Some(node) = ptr.as_ref() {
            len += 1;
            ptr = &node.next;
        }
        if len == 0 {
            return None;
        }
        let target_idx = len.checked_sub(n.try_into().unwrap()).unwrap();
        if target_idx == 0{
            return head.unwrap().next.take();
        }
        let mut target = len.checked_sub(n.try_into().unwrap()).unwrap();
        let mut ptr: &mut Option<Box<ListNode>> = &mut head;
        while let Some(node) = ptr.as_mut() && target > 0 {
            target -= 1;
            if target == 0 {
                let to_remove = node.next.take();
                node.next = to_remove.unwrap().next.take();
                break;
            } else {
                ptr = &mut node.next;
            }
        }
        
        head
    }
}
