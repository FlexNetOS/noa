//! Per-provider rate limit registry (FR-095)

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub max_rps: u32,
    pub burst: u32,
}

static LIMITS: OnceLock<Mutex<HashMap<String, RateLimit>>> = OnceLock::new();

fn registry() -> &'static Mutex<HashMap<String, RateLimit>> {
    LIMITS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(
            "claude".into(),
            RateLimit {
                max_rps: 5,
                burst: 10,
            },
        );
        map.insert(
            "codex".into(),
            RateLimit {
                max_rps: 5,
                burst: 10,
            },
        );
        map.insert(
            "cursor".into(),
            RateLimit {
                max_rps: 10,
                burst: 20,
            },
        );
        map.insert(
            "llama.cpp".into(),
            RateLimit {
                max_rps: 20,
                burst: 40,
            },
        );
        Mutex::new(map)
    })
}

pub fn set_limit(provider: &str, limit: RateLimit) {
    registry().lock().unwrap().insert(provider.to_string(), limit);
}

pub fn get_limit(provider: &str) -> Option<RateLimit> {
    registry().lock().unwrap().get(provider).cloned()
}
