impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();
        if n1 > n2 { return false; }
        let mut s1_counts = s1.bytes().fold([0; 26], |mut acc, ch| {
            acc[(ch - b'a') as usize] += 1;
            acc
        });

        let s2_bytes = s2.as_bytes();
        let mut s2_counts = [0; 26];
        let mut j = 0;
        for i in 0..=(n2-n1) {
            while j-i+1 <= n1 {
                s2_counts[(s2_bytes[j] - b'a') as usize] += 1;
                j += 1;
            }
            if s2_counts == s1_counts {
                return true;
            }
            println!("{:?}, {:?}", s1_counts, s2_counts);
            s2_counts[(s2_bytes[i] - b'a') as usize] -= 1;
        }
        return false;
    }
}
