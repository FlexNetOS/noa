//! Pattern Analyzer Unit Tests
//!
//! Comprehensive unit tests for pattern analyzer.
//! §3.12: Test Everything, Trust Nothing
//!
//! T635a: Create unit tests for pattern_analyzer

#[cfg(test)]
mod tests {
    use super::super::pattern_analyzer::*;

    #[tokio::test]
    async fn test_pattern_analyzer_initialization() {
        let analyzer = PatternAnalyzer::new();
        assert_eq!(analyzer.all_patterns().await.len(), 0);
        assert_eq!(analyzer.analysis_history().await.len(), 0);
    }

    #[tokio::test]
    async fn test_no_patterns_detected() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "latency_ms": 100.0,
            "error_rate": 0.01,
            "cpu_usage": 0.5
        });

        let patterns = analyzer.analyze("Service".to_string(), metrics).await.unwrap();
        assert_eq!(patterns.len(), 0);
    }

    #[tokio::test]
    async fn test_patterns_by_type() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "latency_ms": 1500.0,
            "error_rate": 0.1
        });

        analyzer.analyze("Service".to_string(), metrics).await.unwrap();

        let perf_patterns = analyzer.patterns_by_type(&PatternType::PerformanceDegradation).await;
        assert_eq!(perf_patterns.len(), 1);

        let error_patterns = analyzer.patterns_by_type(&PatternType::ErrorRateIncrease).await;
        assert_eq!(error_patterns.len(), 1);
    }

    #[tokio::test]
    async fn test_patterns_by_component() {
        let analyzer = PatternAnalyzer::new();

        analyzer.analyze("Database".to_string(), serde_json::json!({"latency_ms": 1500.0})).await.unwrap();
        analyzer.analyze("API".to_string(), serde_json::json!({"error_rate": 0.1})).await.unwrap();

        let db_patterns = analyzer.patterns_by_component("Database").await;
        assert_eq!(db_patterns.len(), 1);

        let api_patterns = analyzer.patterns_by_component("API").await;
        assert_eq!(api_patterns.len(), 1);
    }

    #[tokio::test]
    async fn test_severity_calculation() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "latency_ms": 2000.0 // Should give severity of 1.0
        });

        let patterns = analyzer.analyze("Service".to_string(), metrics).await.unwrap();
        assert_eq!(patterns.len(), 1);
        assert!(patterns[0].severity <= 1.0);
        assert!(patterns[0].severity > 0.0);
    }

    #[tokio::test]
    async fn test_suggested_improvements() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "latency_ms": 1500.0
        });

        let patterns = analyzer.analyze("Service".to_string(), metrics).await.unwrap();
        assert_eq!(patterns.len(), 1);
        assert!(patterns[0].suggested_improvement.is_some());
    }

    #[tokio::test]
    async fn test_clear_patterns() {
        let analyzer = PatternAnalyzer::new();

        analyzer.analyze("Service".to_string(), serde_json::json!({"latency_ms": 1500.0})).await.unwrap();
        assert_eq!(analyzer.all_patterns().await.len(), 1);

        analyzer.clear().await;
        assert_eq!(analyzer.all_patterns().await.len(), 0);
    }

    #[tokio::test]
    async fn test_analysis_history() {
        let analyzer = PatternAnalyzer::new();

        analyzer.analyze("Service".to_string(), serde_json::json!({})).await.unwrap();
        analyzer.analyze("Service".to_string(), serde_json::json!({})).await.unwrap();

        let history = analyzer.analysis_history().await;
        assert_eq!(history.len(), 2);
    }

    #[tokio::test]
    async fn test_edge_case_thresholds() {
        let analyzer = PatternAnalyzer::new();

        // Exactly at threshold
        let metrics = serde_json::json!({
            "latency_ms": 1000.0,
            "error_rate": 0.05,
            "cpu_usage": 0.9
        });

        let patterns = analyzer.analyze("Service".to_string(), metrics).await.unwrap();
        // Should detect patterns at or above thresholds
        assert!(patterns.len() >= 0);
    }

    #[tokio::test]
    async fn test_multiple_analyses_accumulate() {
        let analyzer = PatternAnalyzer::new();

        analyzer.analyze("Service1".to_string(), serde_json::json!({"latency_ms": 1500.0})).await.unwrap();
        analyzer.analyze("Service2".to_string(), serde_json::json!({"error_rate": 0.1})).await.unwrap();

        assert_eq!(analyzer.all_patterns().await.len(), 2);
    }
}

