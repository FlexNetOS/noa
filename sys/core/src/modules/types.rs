use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModuleType {
    Binary,
    Package,
    Library,
    Tool,
    Service,
    Agent,
    Microkernel,
}

impl ModuleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleType::Binary => "binary",
            ModuleType::Package => "package",
            ModuleType::Library => "library",
            ModuleType::Tool => "tool",
            ModuleType::Service => "service",
            ModuleType::Agent => "agent",
            ModuleType::Microkernel => "microkernel",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependency {
    pub name: String,
    #[serde(default)]
    pub version_constraint: Option<String>,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub id: String,
    pub name: String,
    pub module_type: ModuleType,
    pub version: String,
    pub hash: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<ModuleDependency>,
    #[serde(default)]
    pub path: Option<PathBuf>,
}

impl ModuleMetadata {
    pub fn new(name: impl Into<String>, module_type: ModuleType, version: impl Into<String>, hash: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            module_type,
            version: version.into(),
            hash: hash.into(),
            capabilities: Vec::new(),
            dependencies: Vec::new(),
            path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModuleLifecycleState {
    Registered,
    Verified,
    Loaded,
    Executing,
    Unloading,
    Archived,
}
