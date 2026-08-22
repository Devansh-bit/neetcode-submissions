use std::collections::{HashMap, VecDeque};

struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    key: i32,
    value: i32,
}

struct LRUCache {
    capacity: i32,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    head: Option<usize>, // LRU
    tail: Option<usize> // MRU
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity,
            map: HashMap::with_capacity(capacity as usize),
            nodes: Vec::with_capacity(capacity as usize),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let index = match self.map.get(&key) {
            Some(&idx) => idx,
            None => return -1,
        };

        let (prev, next, val) = {
            let node = &self.nodes[index];
            (node.prev, node.next, node.value) 
        };

        if next.is_none() {
            return val;
        }
        if let Some(p) = prev {
            self.nodes[p].next = next;
        } else {
            self.head = next;
        }
        if let Some(n) = next {
            self.nodes[n].prev = prev;
        }
        if let Some(t) = self.tail {
            self.nodes[t].next = Some(index);
        }        
        self.nodes[index].prev = self.tail;
        self.nodes[index].next = None;
        self.tail = Some(index);
        val
    }
    pub fn put(&mut self, key: i32, value: i32) {
        // SCENARIO 1: The key already exists
        if let Some(&index) = self.map.get(&key) {
            // Update the value
            self.nodes[index].value = value;
            
            // Move it to the tail (MRU) - this is the exact same pointer logic from get()
            let (prev, next) = (self.nodes[index].prev, self.nodes[index].next);
            
            if next.is_none() { 
                return; // Already at the tail, do nothing
            } 
            
            if let Some(p) = prev { self.nodes[p].next = next; }
            else { self.head = next; }
            
            if let Some(n) = next { self.nodes[n].prev = prev; }
            
            let t = self.tail.unwrap();
            self.nodes[t].next = Some(index);
            self.nodes[index].prev = Some(t);
            self.nodes[index].next = None;
            self.tail = Some(index);
            
            return;
        }

        // SCENARIO 2: New key, but we are at capacity (Recycle)
        if self.nodes.len() == self.capacity as usize {
            let head_idx = self.head.unwrap(); // This is our LRU node
            
            // Remove the old key from the map, insert the new key
            let old_key = self.nodes[head_idx].key;
            self.map.remove(&old_key);
            self.map.insert(key, head_idx);
            
            // Overwrite the physical node's data in place
            self.nodes[head_idx].key = key;
            self.nodes[head_idx].value = value;
            
            // Move this recycled node to the tail (MRU)
            // (The if let safely handles the edge case where capacity == 1)
            if let Some(n) = self.nodes[head_idx].next {
                self.head = Some(n); // The next node in line becomes the new head
                self.nodes[n].prev = None;
                
                let t = self.tail.unwrap();
                self.nodes[t].next = Some(head_idx);
                self.nodes[head_idx].prev = Some(t);
                self.nodes[head_idx].next = None;
                self.tail = Some(head_idx);
            }
            
        // SCENARIO 3: New key, and we have room to grow
        } else {
            let new_idx = self.nodes.len();
            self.map.insert(key, new_idx);
            
            // Actually push a new node into the array
            self.nodes.push(Node {
                key,
                value,
                prev: self.tail,
                next: None, // It's going to be the new tail, so no next
            });
            
            // Wire the old tail to point to this new node
            if let Some(t) = self.tail {
                self.nodes[t].next = Some(new_idx);
            } else {
                // If tail was None, the list was empty, so this is also the head
                self.head = Some(new_idx);
            }
            self.tail = Some(new_idx);
        }
    }

}
