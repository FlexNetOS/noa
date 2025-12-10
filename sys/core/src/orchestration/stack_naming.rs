//! Stack naming (Phase 9 - T278)
pub fn persistent_name(base: &str) -> String {
    format!("mas_{}", base)
}

pub fn disposable_name(base: &str) -> String {
    format!("gen_mas_{}", base)
}
