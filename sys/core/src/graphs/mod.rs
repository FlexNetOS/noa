//! Dynamic Graphs Framework (US8)
//!
//! Implements the dynamic graph surfaces used for self-observation and code
//! improvement. Each specialized graph wraps the shared `DomainGraph`
//! foundation and provides light domain helpers.

pub mod base;
pub mod deg;
pub mod dhg;
pub mod dkg;
pub mod dpeg;
pub mod dpg;
pub mod drg;
pub mod dsecg;
pub mod dsg;
pub mod efg;

pub use base::{
    DomainGraph, DynamicGraph, GraphEdge, GraphEvent, GraphKind, GraphNode, GraphSnapshot,
};
pub use dfg_exports::*;

// Convenience re-exports for specialized graphs
mod dfg_exports {
    pub use crate::graphs::deg::DynamicErrorGraph;
    pub use crate::graphs::dhg::DynamicHardwareGraph;
    pub use crate::graphs::dkg::DynamicKnowledgeGraph;
    pub use crate::graphs::dpeg::DynamicPerformanceGraph;
    pub use crate::graphs::dpg::DynamicProcessGraph;
    pub use crate::graphs::drg::DynamicResourceGraph;
    pub use crate::graphs::dsecg::DynamicSecurityGraph;
    pub use crate::graphs::dsg::DynamicSoftwareGraph;
    pub use crate::graphs::efg::EnvironmentFunctionGraph;
}
