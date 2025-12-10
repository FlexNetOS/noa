//! MicroAgentStack lifecycle (Phase 9 - T276)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackStage {
    Bootstrap,
    Execute,
    Validate,
    Package,
    Archive,
}

pub fn next_stage(current: &StackStage) -> Option<StackStage> {
    match current {
        StackStage::Bootstrap => Some(StackStage::Execute),
        StackStage::Execute => Some(StackStage::Validate),
        StackStage::Validate => Some(StackStage::Package),
        StackStage::Package => Some(StackStage::Archive),
        StackStage::Archive => None,
    }
}
