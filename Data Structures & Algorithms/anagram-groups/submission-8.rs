use itertools::Itertools;


impl Solution {

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .map(|s| {
                let key = s.bytes().fold([0u8; 26], |mut acc, b| {
                    acc[(b - b'a') as usize] += 1;
                    acc
                });
                (key, s)  // stack-allocated key, no heap per string
            })
            .into_group_map()
            .into_values()
            .collect()
    }
}
