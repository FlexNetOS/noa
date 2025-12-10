//! CRM Strangler service (Phase 9 - T287-T290)
use crate::error::Result;

pub enum StranglerMode {
    Shadow,
    WriteThrough,
}

pub struct CrmStranglerService {
    mode: StranglerMode,
}

impl CrmStranglerService {
    pub fn new() -> Self {
        Self {
            mode: StranglerMode::Shadow,
        }
    }

    pub fn set_mode(&mut self, mode: StranglerMode) {
        self.mode = mode;
    }

    pub fn status(&self) -> String {
        match self.mode {
            StranglerMode::Shadow => "shadow".into(),
            StranglerMode::WriteThrough => "write-through".into(),
        }
    }

    pub fn rollback(&mut self) -> Result<()> {
        self.mode = StranglerMode::Shadow;
        Ok(())
    }
}
