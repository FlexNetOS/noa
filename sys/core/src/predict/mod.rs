//! Predictive problem solving engines (3.11 / US8)
//!
//! Provides lightweight predictors for patterns, failures, value, resource
//! allocation, prioritization, and risk.

pub mod patterns;
pub mod failure_analysis;
pub mod value_evaluator;
pub mod resource_predictor;
pub mod priority_predictor;
pub mod risk_assessment;

pub use patterns::{PatternMatch, PatternRecognitionEngine};
pub use failure_analysis::{FailureAnalyzer, FailureInsight};
pub use value_evaluator::ValueEvaluator;
pub use resource_predictor::{ResourceForecast, ResourcePredictor};
pub use priority_predictor::{PriorityPrediction, PriorityPredictor};
pub use risk_assessment::{RiskAssessment, RiskAssessor};
