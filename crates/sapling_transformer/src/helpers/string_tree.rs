use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StringTree {
    pub children: HashMap<String, StringTree>,
    pub id: usize,
}

impl StringTree {
    pub fn new(id: usize) -> Self {
        StringTree {
            children: HashMap::new(),
            id,
        }
    }

    pub fn process_path(&mut self, path: &[Option<String>]) -> Vec<usize> {
        if path.is_empty() {
            return Vec::new();
        }
        match &path[0] {
            Some(s) => {
                let next_id = self.children.len();
                let child = self
                    .children
                    .entry(s.clone())
                    .or_insert_with(|| StringTree::new(next_id));
                let mut ids = vec![child.id];
                ids.extend(child.process_path(&path[1..]));
                ids
            }
            None => Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.children.clear();
    }
}
