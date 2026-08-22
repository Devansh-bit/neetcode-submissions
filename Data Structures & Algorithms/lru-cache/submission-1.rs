use std::collections::HashMap;

struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    key: i32,
    value: i32,
}

pub struct LRUCache {
    capacity: i32,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    head: Option<usize>, // LRU
    tail: Option<usize>  // MRU
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

    // --- NEW HELPER METHOD ---
    fn move_to_tail(&mut self, index: usize) {
        let (prev, next) = (self.nodes[index].prev, self.nodes[index].next);

        // If it's already the tail, we don't need to do anything
        if next.is_none() {
            return;
        }

        // Unlink from current position
        if let Some(p) = prev {
            self.nodes[p].next = next;
        } else {
            self.head = next;
        }

        if let Some(n) = next {
            self.nodes[n].prev = prev;
        }

        // Link to the end (tail)
        let t = self.tail.unwrap();
        self.nodes[t].next = Some(index);
        self.nodes[index].prev = Some(t);
        self.nodes[index].next = None;
        self.tail = Some(index);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let index = match self.map.get(&key) {
            Some(&idx) => idx,
            None => return -1,
        };

        self.move_to_tail(index);
        self.nodes[index].value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // SCENARIO 1: The key already exists
        if let Some(&index) = self.map.get(&key) {
            self.nodes[index].value = value;
            self.move_to_tail(index);
            return;
        }

        // SCENARIO 2: New key, but we are at capacity (Recycle LRU node)
        if self.nodes.len() == self.capacity as usize {
            let head_idx = self.head.unwrap(); 
            
            // Remove old key, insert new key
            let old_key = self.nodes[head_idx].key;
            self.map.remove(&old_key);
            self.map.insert(key, head_idx);
            
            // Update node data
            self.nodes[head_idx].key = key;
            self.nodes[head_idx].value = value;
            
            // Move the recycled node to the MRU position
            self.move_to_tail(head_idx);
            
        // SCENARIO 3: New key, and we have room to grow
        } else {
            let new_idx = self.nodes.len();
            self.map.insert(key, new_idx);
            
            self.nodes.push(Node {
                key,
                value,
                prev: self.tail,
                next: None,
            });
            
            if let Some(t) = self.tail {
                self.nodes[t].next = Some(new_idx);
            } else {
                self.head = Some(new_idx); // First node inserted
            }
            self.tail = Some(new_idx);
        }
    }
}