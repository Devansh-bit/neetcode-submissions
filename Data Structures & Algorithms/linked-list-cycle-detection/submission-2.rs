// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

use std::collections::HashSet;

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut set = HashSet::new();
        let mut current = head;
        unsafe {
            while !current.is_null() {
                if !set.insert(current.clone()) {
                    return true;
                }
                current = (*current).next;
            }
        }
        false
    }
}
