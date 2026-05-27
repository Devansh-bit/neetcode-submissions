struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_stack.last() {
            Some(&v) => if val <= v { self.min_stack.push(val); },
            None => self.min_stack.push(val),
        } 
    }

    pub fn pop(&mut self) {
        if let Some(current) = self.stack.pop() {
            if let Some(&min) = self.min_stack.last() {
                if current <= min {
                    self.min_stack.pop();
                }
            }
        }
    }

    pub fn top(&self) -> i32 {
        if let Some(&current) = self.stack.last() {
            return current;
        }
        unreachable!();
    }

    pub fn get_min(&self) -> i32 {
        if let Some(&min) = self.min_stack.last() {
            return min;
        }
        unreachable!();
    }
}
