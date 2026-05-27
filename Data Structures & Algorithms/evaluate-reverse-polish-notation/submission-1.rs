impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in tokens {
            if let Ok(number) = token.parse::<i32>() {
                stack.push(number);
                continue;
            }
            let op2 = stack.pop().unwrap();
            let op1 = stack.pop().unwrap();
            stack.push(
                match token.as_str() {
                    "+" => op1 + op2,
                    "-" => op1 - op2,
                    "*" => op1 * op2,
                    "/" => op1 / op2,
                    _ => unreachable!(),
                }
            )
        }
        stack.pop().unwrap()
    }
}
