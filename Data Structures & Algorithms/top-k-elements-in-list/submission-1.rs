impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();

        let mut counts: HashMap<i32, usize> = HashMap::with_capacity(n);
        for num in nums.iter() {
            *counts.entry(*num).or_default() += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n+1];

        for (num, freq) in counts {
            buckets[freq].push(num);
        }

        buckets
            .into_iter()
            .rev()
            .flatten()
            .take(k as usize)
            .collect()
        
    }
}
