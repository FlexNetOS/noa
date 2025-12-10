use std::collections::HashMap;

/// KDIR_CAP: Canonical directory for capsules.
#[derive(Default, Debug, Clone)]
pub struct KnowledgeDirectory {
    entries: HashMap<String, Vec<String>>,
}

impl KnowledgeDirectory {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, category: impl Into<String>, path: impl Into<String>) {
        let category = category.into();
        let path = path.into();
        self.entries.entry(category).or_default().push(path);
    }

    pub fn list(&self, category: &str) -> Vec<String> {
        self.entries
            .get(category)
            .cloned()
            .unwrap_or_default()
    }

    pub fn all(&self) -> HashMap<String, Vec<String>> {
        self.entries.clone()
    }
}
