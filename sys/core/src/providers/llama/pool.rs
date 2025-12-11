//! llama.cpp model pool (5+ concurrent)

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ModelInstance {
    pub name: String,
    pub loaded: bool,
}

pub struct ModelPool {
    pub instances: Vec<ModelInstance>,
}

impl ModelPool {
    pub fn new() -> Self {
        Self {
            instances: (0..5)
                .map(|i| ModelInstance {
                    name: format!("llama-model-{}", i + 1),
                    loaded: true,
                })
                .collect(),
        }
    }

    pub fn acquire(&self) -> Option<ModelInstance> {
        self.instances.first().cloned()
    }

    pub fn status(&self) -> Result<usize> {
        Ok(self.instances.len())
    }
}
