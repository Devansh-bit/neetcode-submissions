use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(entries) = self.map.get(&key) else {
            return String::new();
        };

        match entries.binary_search_by_key(&timestamp, |&(_, t)| t) {
            Ok(idx) => entries[idx].0.clone(),
            Err(0) => String::new(),
            Err(idx) => entries[idx - 1].0.clone(),
        }
    }
}
