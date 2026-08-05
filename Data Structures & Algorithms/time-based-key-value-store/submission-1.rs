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
        if let Some(val) = self.map.get(&key) {
            match val.binary_search_by(|x| x.1.cmp(&timestamp)) {
                Ok(idx) => return val[idx].0.clone(),
                Err(idx) => {
                    let res = &val[idx.saturating_sub(1)];
                    if res.1 <= timestamp {
                        return res.0.clone();
                    } else {
                        return String::new();
                    }
                }
            }  
        } else {
            return String::new();
        }
    }
}
