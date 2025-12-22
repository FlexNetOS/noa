//! Automation Module
//!
//! Advanced automation features including code review, deployment, and workflow orchestration

pub mod code_review;
pub mod deployment;
pub mod knowledge_base;

pub use code_review::AutomatedCodeReview;
pub use deployment::DeploymentAutomation;
pub use knowledge_base::KnowledgeBaseInterrogation;
