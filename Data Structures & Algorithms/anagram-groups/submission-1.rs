impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut output: Vec<Vec<String>> = vec![];
        let mut seen = HashMap::new();
        let mut current_index = 0;
        for word in strs.into_iter() {
            let mut arr = [0; 26];
            for c in word.bytes() {
                arr[(c - b'a') as usize] += 1;
            }
            if let Some(&out_index) = seen.get(&arr) {
                output[out_index as usize].push(word);
            } else {
                seen.insert(arr, current_index);
                output.push(vec![word]);
                current_index += 1;
            }
        }
        output
    }
}
