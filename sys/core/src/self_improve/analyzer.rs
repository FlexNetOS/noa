use crate::self_improve::metrics::PerformanceMetrics;

/// Efficiency findings derived from observed metrics.
#[derive(Debug, Clone)]
pub struct EfficiencyReport {
    pub bottlenecks: Vec<String>,
    pub opportunities: Vec<String>,
    pub score: f64,
}

impl EfficiencyReport {
    pub fn empty() -> Self {
        Self {
            bottlenecks: Vec::new(),
            opportunities: Vec::new(),
            score: 1.0,
        }
    }
}

/// Analyzer that converts metrics into actionable signals.
pub struct EfficiencyAnalyzer {
    latency_threshold_ms: f64,
    success_threshold: f64,
    resource_threshold: f64,
}

impl EfficiencyAnalyzer {
    pub fn new(latency_threshold_ms: f64, success_threshold: f64, resource_threshold: f64) -> Self {
        Self {
            latency_threshold_ms,
            success_threshold,
            resource_threshold,
        }
    }

    pub fn analyze(&self, metrics: &PerformanceMetrics) -> EfficiencyReport {
        let mut report = EfficiencyReport::empty();

        if let Some(avg_latency) = metrics.average("latency_ms") {
            if avg_latency > self.latency_threshold_ms {
                report.bottlenecks.push(format!(
                    "Latency {:.0}ms exceeds threshold {:.0}ms",
                    avg_latency, self.latency_threshold_ms
                ));
                report.score -= 0.2;
            } else {
                report.opportunities.push("Latency within target".to_string());
            }
        }

        if let Some(success_rate) = metrics.latest("success_rate").map(|s| s.value) {
            if success_rate < self.success_threshold {
                report.bottlenecks.push(format!(
                    "Success rate {:.2}% below {:.2}%",
                    success_rate * 100.0,
                    self.success_threshold * 100.0
                ));
                report.score -= 0.3;
            } else {
                report.opportunities.push("Success rate healthy".to_string());
            }
        }

        if let Some(utilization) = metrics.latest("resource_utilization").map(|s| s.value) {
            if utilization > self.resource_threshold {
                report.bottlenecks.push(format!(
                    "Resource utilization {:.0}% saturated",
                    utilization * 100.0
                ));
                report.score -= 0.2;
            } else {
                report.opportunities.push("Resource headroom available".to_string());
            }
        }

        if report.score < 0.0 {
            report.score = 0.0;
        }

        report
    }
}

impl Default for EfficiencyAnalyzer {
    fn default() -> Self {
        Self::new(500.0, 0.95, 0.85)
    }
}
