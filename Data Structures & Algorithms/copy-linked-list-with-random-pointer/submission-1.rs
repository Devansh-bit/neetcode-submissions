// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn copy_random_list(mut head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut map = HashMap::new();
        let mut ptr = head.clone();
        while let Some(node) = ptr {
            let clone = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            map.insert(Rc::as_ptr(&node), clone);
            ptr = node.borrow().next.clone();
        }

        let mut ptr = head.clone();
        while let Some(node) = ptr {
            let clone = &map[&Rc::as_ptr(&node)];
            let original = node.borrow();

            clone.borrow_mut().next = original.next.as_ref().map(|n| map[&Rc::as_ptr(n)].clone());
            clone.borrow_mut().random = original.random.as_ref().map(|n| map[&Rc::as_ptr(n)].clone());

            ptr = original.next.clone();
        }

        head.as_ref().map(|n| map[&Rc::as_ptr(n)].clone())
    }
}
