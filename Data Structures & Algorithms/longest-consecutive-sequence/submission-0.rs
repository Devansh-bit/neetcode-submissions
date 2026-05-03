impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // let mut seen_smaller = HashSet::new();
        let mut mp = HashMap::new();
        let mut res = 0;

        for num in nums {
            if mp.contains_key(&num) { continue; }
            let left = *mp.get(&(num-1)).unwrap_or(&0);
            let right = *mp.get(&(num+1)).unwrap_or(&0);
            let length = left + right + 1;
            mp.insert(num, length);
            mp.insert(num-left, length);
            mp.insert(num+right, length);
            res = res.max(length);
        }

        res
    }
}
