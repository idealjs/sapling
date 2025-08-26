use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BitMask {
    key_to_map: HashMap<String, u64>,
    count: usize,
}

impl BitMask {
    pub fn new() -> Self {
        Self {
            key_to_map: HashMap::new(),
            count: 0,
        }
    }
    pub fn add_key(&mut self, key: String) -> u64 {
        if let Some(&mask) = self.key_to_map.get(&key) {
            return mask;
        }
        let mask = 1u64 << self.count;
        self.key_to_map.insert(key, mask);
        self.count += 1;
        mask
    }
    pub fn get_mask(&self, key: &str) -> Option<u64> {
        self.key_to_map.get(key).copied()
    }
    pub fn keys(&self) -> Vec<&String> {
        self.key_to_map.keys().collect()
    }
}
