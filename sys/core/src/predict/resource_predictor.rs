/// Forecast of resource utilization.
#[derive(Debug, Clone)]
pub struct ResourceForecast {
    pub cpu: f64,
    pub memory: f64,
    pub horizon_minutes: u64,
}

/// Predicts future resource allocation using moving average.
pub struct ResourcePredictor {
    window: usize,
}

impl ResourcePredictor {
    pub fn new(window: usize) -> Self {
        Self { window }
    }

    pub fn forecast(
        &self,
        cpu_history: &[f64],
        mem_history: &[f64],
        horizon_minutes: u64,
    ) -> ResourceForecast {
        let cpu_avg = Self::avg(cpu_history, self.window);
        let mem_avg = Self::avg(mem_history, self.window);
        ResourceForecast {
            cpu: cpu_avg,
            memory: mem_avg,
            horizon_minutes,
        }
    }

    fn avg(values: &[f64], window: usize) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        let start = values.len().saturating_sub(window);
        let slice = &values[start..];
        slice.iter().sum::<f64>() / slice.len() as f64
    }
}

impl Default for ResourcePredictor {
    fn default() -> Self {
        Self::new(5)
    }
}
