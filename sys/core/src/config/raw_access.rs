use crate::error::{NoaError, Result};

pub fn ptr<'a>(raw: &'a serde_json::Value, pointer: &str) -> Option<&'a serde_json::Value> {
    raw.pointer(pointer)
}

pub fn get_str(raw: &serde_json::Value, pointers: &[&str]) -> Option<String> {
    for p in pointers {
        if let Some(v) = raw.pointer(p).and_then(|v| v.as_str()) {
            return Some(v.to_string());
        }
    }
    None
}

pub fn get_u32(raw: &serde_json::Value, pointers: &[&str]) -> Option<u32> {
    for p in pointers {
        if let Some(v) = raw.pointer(p).and_then(|v| v.as_u64()) {
            return Some(v as u32);
        }
    }
    None
}

pub fn get_f32(raw: &serde_json::Value, pointers: &[&str]) -> Option<f32> {
    for p in pointers {
        if let Some(v) = raw.pointer(p).and_then(|v| v.as_f64()) {
            return Some(v as f32);
        }
    }
    None
}

pub fn require_str(raw: &serde_json::Value, pointers: &[&str], field: &'static str) -> Result<String> {
    get_str(raw, pointers).ok_or_else(|| {
        NoaError::Validation(crate::error::ValidationError::new(
            field,
            format!("Missing configs value (any of): {}", pointers.join(", ")),
            "MISSING_configs",
        ))
    })
}
