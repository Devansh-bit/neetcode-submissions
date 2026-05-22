impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut table = HashMap::with_capacity(128);
        let mut max_len = 0;

        for (right, &byte) in s.as_bytes().iter().enumerate() {
            table.entry(byte)
                .and_modify(|v|  {
                    left = left.max(*v+1);
                    *v = right;
                })
                .or_insert(right);
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}