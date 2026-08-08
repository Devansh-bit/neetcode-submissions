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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return match (list1, list2) {
            (None, None) => return None,
            (Some(n1), None) => return Some(n1),
            (None, Some(n2)) => return Some(n2),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    let n1_next = n1.next;
                    n1.next = Self::merge_two_lists(n1_next, Some(n2));
                    Some(n1)
                } else {
                    let n2_next = n2.next;
                    n2.next = Self::merge_two_lists(n2_next, Some(n1));
                    Some(n2)
                }
            }
        }
        
    }
}
