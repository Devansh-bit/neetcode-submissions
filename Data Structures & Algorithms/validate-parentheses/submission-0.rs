impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for ch in s.chars() {
            println!("{}", ch);
            if ch == '(' || ch == '{' || ch == '[' {
                stack.push(ch);
            } else if let Some(top) = stack.pop() {
                if (top == '(' && ch != ')') || (top == '{' && ch != '}') || (top == '[' && ch != ']') {
                    return false;
                }
            } else { return false; }
            println!("{:?}", stack);
        }
        stack.len() == 0
    }
}
