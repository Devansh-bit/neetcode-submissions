impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::new();
        for word in strs {
            result.push_str(&word.len().to_string());
            result.push('#');
            result.push_str(&word);
        }
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = vec![];
        let mut i = 0;

        while i < s.len() {
            let j = s[i..].find('#').unwrap() + i;
            let len: usize = s[i..j].parse().unwrap();
            let word_start = j+1;
            let word_end = word_start+len;
            result.push(s[word_start..word_end].to_string());
            i = word_end;
        }
        result
    }
}
