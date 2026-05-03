impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut buckets = vec![vec![]; n+1];
        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }
        for (num, count) in counts {
            buckets[count as usize].push(num);
        }

        let mut result = vec![];

        for bucket in buckets.into_iter().rev() {
            for num in bucket {
                result.push(num);
                if result.len() == k as usize {
                    return result;
                }
            }
        }

        result
    }
}
