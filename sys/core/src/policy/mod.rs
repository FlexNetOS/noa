//! Evidence-based execution policy (US8)
//!
//! Contains policy rules: truth sources, hard stop, triple verify, gap hunt.

pub mod gap_hunt;
pub mod hard_stop;
pub mod triple_verify;
pub mod truth_sources;

pub use gap_hunt::{GapHuntFinding, GapHuntRule};
pub use hard_stop::HardStopRule;
pub use triple_verify::{TripleVerifyRule, VerificationEvidence};
pub use truth_sources::{TruthSource, TruthSourceOrder, TruthSourcePolicy};
