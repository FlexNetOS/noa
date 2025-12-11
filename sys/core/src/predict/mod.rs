//! Predictive problem solving engines (3.11 / US8)
//!
//! Provides lightweight predictors for patterns, failures, value, resource
//! allocation, prioritization, and risk.

pub mod failure_analysis;
pub mod patterns;
pub mod priority_predictor;
pub mod resource_predictor;
pub mod risk_assessment;
pub mod value_evaluator;

pub use failure_analysis::{FailureAnalyzer, FailureInsight};
pub use patterns::{PatternMatch, PatternRecognitionEngine};
pub use priority_predictor::{PriorityPrediction, PriorityPredictor};
pub use resource_predictor::{ResourceForecast, ResourcePredictor};
pub use risk_assessment::{RiskAssessment, RiskAssessor};
pub use value_evaluator::ValueEvaluator;
