//! MAML (Model-Agnostic Meta-Learning) Module
//!
//! Few-shot learning through meta-learning

pub mod few_shot;
pub mod inner_loop;
pub mod outer_loop;

pub use few_shot::FewShotLearner;
pub use inner_loop::InnerLoopAdapter;
pub use outer_loop::OuterLoopOptimizer;
