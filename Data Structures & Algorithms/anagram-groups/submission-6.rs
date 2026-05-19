use itertools::Itertools;


impl Solution {

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .map(|s| {
                let mut key: Vec<u8> = s.bytes().collect();
                key.sort_unstable();
                (key, s)
            })
            .into_group_map()   // HashMap<Key, Vec<Value>> from (K, V) iterator
            .into_values()
            .collect()
    }
}
