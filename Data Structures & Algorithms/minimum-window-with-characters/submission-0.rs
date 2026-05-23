impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();

        if t.len() > s.len() || t.is_empty() { return String::new(); }

        let mut t_counts = t.iter().copied().fold([0i32; 256], |mut acc, ch| {
            acc[ch as usize] += 1;
            acc
        });

        let mut required = t.len() as i32;
        let mut min_len = usize::MAX;
        let mut min_start = 0;
        let mut left = 0;

        for right in 0..s.len() {
            let rch = s[right] as usize;
            if t_counts[rch] > 0 {
                required -= 1;
            }
            t_counts[rch] -= 1;

            while required == 0 {
                if right - left + 1 < min_len {
                    min_len = right - left + 1;
                    min_start = left;
                }
                let lch = s[left] as usize;
                t_counts[lch] += 1;

                if t_counts[lch] > 0 {
                    required += 1;
                }
                left += 1;
            }
        }
        
        if min_len == usize::MAX || min_start+min_len > s.len() {
            return String::new();
        }
        String::from_utf8(s[min_start..(min_start+min_len)].to_vec()).expect("")
    }
}
