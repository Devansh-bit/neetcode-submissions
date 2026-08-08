impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>, 
        mut list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        // 1. Create a dummy node on the stack
        let mut dummy = ListNode::new(0);
        
        // 2. Create a mutable reference 'tail' pointing to the dummy
        let mut tail = &mut dummy;

        // 3. Peek at the values without moving them yet
        while let (Some(n1), Some(n2)) = (list1.as_ref(), list2.as_ref()) {
            if n1.val < n2.val {
                // Move the whole box out of list1
                let mut node = list1.unwrap(); 
                
                // Disconnect the rest of the list and put it back in list1
                list1 = node.next.take(); 
                
                // Attach the single node to our merged list
                tail.next = Some(node);
            } else {
                let mut node = list2.unwrap();
                list2 = node.next.take();
                tail.next = Some(node);
            }
            // 4. Advance the tail pointer to the newly added node
            tail = tail.next.as_deref_mut().unwrap();
        }

        // 5. Attach whatever is left of either list
        tail.next = list1.or(list2);

        // 6. Return everything after the dummy node
        dummy.next
    }
}