use crate::modules::types::ModuleLifecycleState;

/// Check whether a lifecycle transition is valid.
pub fn is_valid_transition(from: &ModuleLifecycleState, to: &ModuleLifecycleState) -> bool {
    use ModuleLifecycleState::*;
    match (from, to) {
        (Registered, Verified)
        | (Verified, Loaded)
        | (Loaded, Executing)
        | (Executing, Unloading)
        | (Unloading, Archived)
        | (Verified, Archived)
        | (Loaded, Archived) => true,
        _ => false,
    }
}

/// Transition helper that enforces allowed transitions.
pub fn transition(
    current: ModuleLifecycleState,
    target: ModuleLifecycleState,
) -> Result<ModuleLifecycleState, String> {
    if is_valid_transition(&current, &target) {
        Ok(target)
    } else {
        Err(format!(
            "invalid module lifecycle transition: {:?} -> {:?}",
            current, target
        ))
    }
}
