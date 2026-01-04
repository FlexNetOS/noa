use crate::error::{NoaError, Result};

#[derive(Debug, Clone)]
pub enum Query {
    Ptr(String),
    Env(String),
    Coalesce(Vec<Query>),
    Literal(serde_json::Value),
    AsStr(Box<Query>),
    AsU32(Box<Query>),
    AsF32(Box<Query>),
}

impl Query {
    pub fn eval(&self, raw: &serde_json::Value) -> Result<Option<serde_json::Value>> {
        match self {
            Query::Ptr(p) => Ok(raw.pointer(p).cloned()),
            Query::Env(name) => Ok(std::env::var(name).ok().map(serde_json::Value::String)),
            Query::Literal(v) => Ok(Some(v.clone())),
            Query::Coalesce(items) => {
                for q in items {
                    if let Some(v) = q.eval(raw)? {
                        if !v.is_null() {
                            return Ok(Some(v));
                        }
                    }
                }
                Ok(None)
            }
            Query::AsStr(q) => {
                let v = q.eval(raw)?;
                Ok(v.and_then(|v| v.as_str().map(|s| serde_json::Value::String(s.to_string()))))
            }
            Query::AsU32(q) => {
                let v = q.eval(raw)?;
                Ok(v.and_then(|v| v.as_u64().map(|n| serde_json::Value::Number((n as u64).into()))))
            }
            Query::AsF32(q) => {
                let v = q.eval(raw)?;
                let n = v.and_then(|v| v.as_f64());
                Ok(n.and_then(|f| serde_json::Number::from_f64(f).map(serde_json::Value::Number)))
            }
        }
    }

    pub fn eval_str(&self, raw: &serde_json::Value) -> Result<Option<String>> {
        Ok(self
            .eval(raw)?
            .and_then(|v| v.as_str().map(|s| s.to_string())))
    }

    pub fn eval_u32(&self, raw: &serde_json::Value) -> Result<Option<u32>> {
        Ok(self.eval(raw)?.and_then(|v| v.as_u64().map(|n| n as u32)))
    }

    pub fn eval_f32(&self, raw: &serde_json::Value) -> Result<Option<f32>> {
        Ok(self.eval(raw)?.and_then(|v| v.as_f64().map(|n| n as f32)))
    }
}

#[derive(Debug, Clone)]
pub struct CompiledQuery {
    pub key: String,
    pub query: Query,
}

impl CompiledQuery {
    pub fn get_str(&self, raw: &serde_json::Value) -> Result<Option<String>> {
        self.query.eval_str(raw)
    }

    pub fn get_u32(&self, raw: &serde_json::Value) -> Result<Option<u32>> {
        self.query.eval_u32(raw)
    }

    pub fn get_f32(&self, raw: &serde_json::Value) -> Result<Option<f32>> {
        self.query.eval_f32(raw)
    }
}

pub fn compile_from_raw(raw: &serde_json::Value, key: &str) -> Result<Option<CompiledQuery>> {
    let q = raw.pointer(key).cloned();
    let Some(q) = q else { return Ok(None); };

    let query = parse_query(&q).map_err(|e| NoaError::Validation(crate::error::ValidationError::new(
        "configs_query",
        e,
        "INVALID_configs_QUERY",
    )))?;

    Ok(Some(CompiledQuery { key: key.to_string(), query }))
}

fn parse_query(v: &serde_json::Value) -> std::result::Result<Query, String> {
    if let Some(s) = v.as_str() {
        // Shorthand: strings starting with '/' are pointers
        if s.starts_with('/') {
            return Ok(Query::Ptr(s.to_string()));
        }
        return Ok(Query::Literal(serde_json::Value::String(s.to_string())));
    }

    if !v.is_object() {
        return Ok(Query::Literal(v.clone()));
    }

    let obj = v.as_object().ok_or("query must be an object")?;
    let op = obj
        .get("op")
        .and_then(|v| v.as_str())
        .ok_or("query missing 'op'")?;

    let args = obj.get("args");

    match op {
        "ptr" => {
            let p = args
                .and_then(|a| a.as_array())
                .and_then(|a| a.get(0))
                .and_then(|v| v.as_str())
                .ok_or("ptr expects args[0] string")?;
            Ok(Query::Ptr(p.to_string()))
        }
        "env" => {
            let n = args
                .and_then(|a| a.as_array())
                .and_then(|a| a.get(0))
                .and_then(|v| v.as_str())
                .ok_or("env expects args[0] string")?;
            Ok(Query::Env(n.to_string()))
        }
        "coalesce" => {
            let arr = args
                .and_then(|a| a.as_array())
                .ok_or("coalesce expects args array")?;
            let mut out = Vec::new();
            for item in arr {
                out.push(parse_query(item)?);
            }
            Ok(Query::Coalesce(out))
        }
        "str" => {
            let inner = args
                .and_then(|a| a.as_array())
                .and_then(|a| a.get(0))
                .ok_or("str expects args[0]")?;
            Ok(Query::AsStr(Box::new(parse_query(inner)?)))
        }
        "u32" => {
            let inner = args
                .and_then(|a| a.as_array())
                .and_then(|a| a.get(0))
                .ok_or("u32 expects args[0]")?;
            Ok(Query::AsU32(Box::new(parse_query(inner)?)))
        }
        "f32" => {
            let inner = args
                .and_then(|a| a.as_array())
                .and_then(|a| a.get(0))
                .ok_or("f32 expects args[0]")?;
            Ok(Query::AsF32(Box::new(parse_query(inner)?)))
        }
        _ => Err(format!("unknown op: {}", op)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesce_ptr_literal() {
        let raw = serde_json::json!({
            "memory": {"search": {"embedding_model": "m1"}}
        });

        let q = Query::Coalesce(vec![
            Query::Ptr("/memory/search/embedding_model".to_string()),
            Query::Literal(serde_json::Value::String("fallback".to_string())),
        ]);

        assert_eq!(q.eval_str(&raw).unwrap().unwrap(), "m1");
    }

    #[test]
    fn test_parse_query_ptr_shorthand() {
        let qv = serde_json::Value::String("/memory/search/default_limit".to_string());
        let q = parse_query(&qv).unwrap();
        match q {
            Query::Ptr(p) => assert_eq!(p, "/memory/search/default_limit"),
            _ => panic!("expected ptr"),
        }
    }
}
