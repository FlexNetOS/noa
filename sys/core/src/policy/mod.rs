//! Evidence-based execution policy (US8)
//!
//! Contains policy rules: truth sources, hard stop, triple verify, gap hunt.

pub mod truth_sources;
pub mod hard_stop;
pub mod triple_verify;
pub mod gap_hunt;

pub use truth_sources::{TruthSource, TruthSourceOrder, TruthSourcePolicy};
pub use hard_stop::HardStopRule;
pub use triple_verify::{VerificationEvidence, TripleVerifyRule};
pub use gap_hunt::{GapHuntFinding, GapHuntRule};
