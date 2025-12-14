//! Context lifecycle management

use crate::error::Result;
use crate::providers::shared_memory::{SharedContext, SharedProviderMemory};

pub struct ContextManager {
    memory: SharedProviderMemory,
}

impl ContextManager {
    pub fn new(memory: SharedProviderMemory) -> Self {
        Self { memory }
    }

    pub fn create_context(
        &self,
        provider: &str,
        context_type: &str,
        content: &str,
        metadata: Option<String>,
        session_id: Option<String>,
    ) -> Result<()> {
        let ctx = SharedContext {
            provider: provider.to_string(),
            context_type: context_type.to_string(),
            content: content.to_string(),
            metadata,
            session_id,
        };
        self.memory.upsert(ctx)
    }
}
