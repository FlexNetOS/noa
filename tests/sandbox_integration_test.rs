//! Integration tests for agent sandbox capsules.
//!
//! Tests capsule configsuration loading, validation, and policy enforcement.

use std::fs;
use std::path::Path;
use serde_json::Value;

/// Load and parse a capsule configsuration.
fn load_capsule(name: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("sandbox/agents/capsules")
        .join(format!("{}.json", name));
    
    let content = fs::read_to_string(&path)?;
    let capsule: Value = serde_json::from_str(&content)?;
    Ok(capsule)
}

/// Load runtime limits configsuration.
fn load_limits() -> Result<Value, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("sandbox/agents/runtime/limits.json");
    
    let content = fs::read_to_string(&path)?;
    let limits: Value = serde_json::from_str(&content)?;
    Ok(limits)
}

/// Load runtime permissions configsuration.
fn load_permissions() -> Result<Value, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("sandbox/agents/runtime/permissions.json");
    
    let content = fs::read_to_string(&path)?;
    let permissions: Value = serde_json::from_str(&content)?;
    Ok(permissions)
}

#[cfg(test)]
mod capsule_tests {
    use super::*;

    #[test]
    fn test_base_capsule_structure() {
        let capsule = load_capsule("base").expect("Failed to load base capsule");
        
        // Verify required sections exist
        assert!(capsule.get("name").is_some());
        assert!(capsule.get("version").is_some());
        assert!(capsule.get("resources").is_some());
        assert!(capsule.get("network").is_some());
        assert!(capsule.get("filesystem").is_some());
        assert!(capsule.get("capabilities").is_some());
        assert!(capsule.get("security").is_some());
    }

    #[test]
    fn test_base_capsule_resources() {
        let capsule = load_capsule("base").expect("Failed to load base capsule");
        let resources = &capsule["resources"];
        
        // Verify resource limits
        assert!(resources["max_memory_mb"].as_u64().unwrap() > 0);
        assert!(resources["max_cpu_percent"].as_u64().unwrap() <= 100);
        assert!(resources["max_disk_mb"].as_u64().unwrap() > 0);
        assert!(resources["max_execution_time_secs"].as_u64().unwrap() > 0);
    }

    #[test]
    fn test_code_agent_capsule() {
        let capsule = load_capsule("code-agent").expect("Failed to load code-agent capsule");
        
        assert_eq!(capsule["name"].as_str().unwrap(), "code-agent");
        
        // Code agent should have write access
        let fs = &capsule["filesystem"];
        assert!(fs["allowed_paths"].as_array().unwrap().len() > 0);
        assert_eq!(fs["write_access"].as_bool().unwrap(), true);
        
        // Code agent should have terminal access
        let caps = &capsule["capabilities"];
        assert_eq!(caps["terminal"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_chat_agent_capsule() {
        let capsule = load_capsule("chat-agent").expect("Failed to load chat-agent capsule");
        
        assert_eq!(capsule["name"].as_str().unwrap(), "chat-agent");
        
        // Chat agent should be read-only
        let fs = &capsule["filesystem"];
        assert_eq!(fs["write_access"].as_bool().unwrap(), false);
        
        // Chat agent should NOT have terminal access
        let caps = &capsule["capabilities"];
        assert_eq!(caps["terminal"].as_bool().unwrap(), false);
    }

    #[test]
    fn test_task_agent_capsule() {
        let capsule = load_capsule("task-agent").expect("Failed to load task-agent capsule");
        
        assert_eq!(capsule["name"].as_str().unwrap(), "task-agent");
        
        // Task agent should have full capabilities
        let caps = &capsule["capabilities"];
        assert_eq!(caps["read_files"].as_bool().unwrap(), true);
        assert_eq!(caps["write_files"].as_bool().unwrap(), true);
        assert_eq!(caps["terminal"].as_bool().unwrap(), true);
        assert_eq!(caps["network"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_capsule_security_policies() {
        for name in ["base", "code-agent", "chat-agent", "task-agent"] {
            let capsule = load_capsule(name).expect(&format!("Failed to load {} capsule", name));
            let security = &capsule["security"];
            
            // All capsules should have security section
            assert!(security.get("sandbox_type").is_some());
            assert!(security.get("allow_network_to_localhost").is_some());
            assert!(security.get("allow_env_vars").is_some());
        }
    }

    #[test]
    fn test_capsule_network_policies() {
        let base = load_capsule("base").expect("Failed to load base capsule");
        let network = &base["network"];
        
        // Verify network policy structure
        assert!(network.get("allowed_hosts").is_some());
        assert!(network.get("blocked_hosts").is_some());
        assert!(network.get("max_connections").is_some());
    }
}

#[cfg(test)]
mod limits_tests {
    use super::*;

    #[test]
    fn test_limits_structure() {
        let limits = load_limits().expect("Failed to load limits");
        
        // Should have agent type sections
        assert!(limits.get("default").is_some());
        assert!(limits.get("code-agent").is_some());
        assert!(limits.get("chat-agent").is_some());
        assert!(limits.get("task-agent").is_some());
    }

    #[test]
    fn test_default_limits() {
        let limits = load_limits().expect("Failed to load limits");
        let default = &limits["default"];
        
        assert!(default["cpu_percent"].as_u64().is_some());
        assert!(default["memory_mb"].as_u64().is_some());
        assert!(default["disk_mb"].as_u64().is_some());
        assert!(default["time_secs"].as_u64().is_some());
    }

    #[test]
    fn test_task_agent_has_higher_limits() {
        let limits = load_limits().expect("Failed to load limits");
        
        let default_mem = limits["default"]["memory_mb"].as_u64().unwrap();
        let task_mem = limits["task-agent"]["memory_mb"].as_u64().unwrap();
        
        // Task agent should have higher or equal memory
        assert!(task_mem >= default_mem);
    }

    #[test]
    fn test_chat_agent_has_lower_limits() {
        let limits = load_limits().expect("Failed to load limits");
        
        let default_cpu = limits["default"]["cpu_percent"].as_u64().unwrap();
        let chat_cpu = limits["chat-agent"]["cpu_percent"].as_u64().unwrap();
        
        // Chat agent should have lower or equal CPU
        assert!(chat_cpu <= default_cpu);
    }
}

#[cfg(test)]
mod permissions_tests {
    use super::*;

    #[test]
    fn test_permissions_structure() {
        let permissions = load_permissions().expect("Failed to load permissions");
        
        // Should have capability definitions
        assert!(permissions.get("capabilities").is_some());
        assert!(permissions.get("roles").is_some());
    }

    #[test]
    fn test_capability_definitions() {
        let permissions = load_permissions().expect("Failed to load permissions");
        let caps = &permissions["capabilities"];
        
        // Core capabilities should be defined
        assert!(caps.get("read_files").is_some());
        assert!(caps.get("write_files").is_some());
        assert!(caps.get("terminal").is_some());
        assert!(caps.get("network").is_some());
    }

    #[test]
    fn test_role_definitions() {
        let permissions = load_permissions().expect("Failed to load permissions");
        let roles = &permissions["roles"];
        
        // Should have role hierarchy
        assert!(roles.get("readonly").is_some());
        assert!(roles.get("developer").is_some());
        assert!(roles.get("admin").is_some());
    }

    #[test]
    fn test_admin_has_all_capabilities() {
        let permissions = load_permissions().expect("Failed to load permissions");
        let admin = &permissions["roles"]["admin"];
        let capabilities = admin["capabilities"].as_array().unwrap();
        
        // Admin should have many capabilities
        assert!(capabilities.len() >= 4);
    }

    #[test]
    fn test_readonly_restrictions() {
        let permissions = load_permissions().expect("Failed to load permissions");
        let readonly = &permissions["roles"]["readonly"];
        let capabilities = readonly["capabilities"].as_array().unwrap();
        
        // Readonly should have limited capabilities
        let cap_strs: Vec<&str> = capabilities.iter()
            .map(|c| c.as_str().unwrap())
            .collect();
        
        assert!(cap_strs.contains(&"read_files"));
        assert!(!cap_strs.contains(&"write_files"));
        assert!(!cap_strs.contains(&"terminal"));
    }
}

#[cfg(test)]
mod policy_enforcement_tests {
    use super::*;

    /// Check if a capsule allows a specific capability.
    fn capsule_allows(capsule: &Value, capability: &str) -> bool {
        capsule["capabilities"][capability].as_bool().unwrap_or(false)
    }

    /// Check if a capsule has network access to a host.
    fn can_access_host(capsule: &Value, host: &str) -> bool {
        let allowed = capsule["network"]["allowed_hosts"].as_array()
            .map(|a| a.iter().any(|h| h.as_str() == Some(host) || h.as_str() == Some("*")))
            .unwrap_or(false);
        
        let blocked = capsule["network"]["blocked_hosts"].as_array()
            .map(|a| a.iter().any(|h| h.as_str() == Some(host)))
            .unwrap_or(false);
        
        allowed && !blocked
    }

    #[test]
    fn test_code_agent_cannot_access_external() {
        let capsule = load_capsule("code-agent").expect("Failed to load code-agent");
        
        // Code agent should not access arbitrary external hosts
        assert!(!can_access_host(&capsule, "malicious-site.com"));
    }

    #[test]
    fn test_chat_agent_readonly_enforcement() {
        let capsule = load_capsule("chat-agent").expect("Failed to load chat-agent");
        
        // Chat agent should not have write or terminal
        assert!(!capsule_allows(&capsule, "write_files"));
        assert!(!capsule_allows(&capsule, "terminal"));
    }

    #[test]
    fn test_capability_isolation() {
        let chat = load_capsule("chat-agent").expect("Failed to load chat-agent");
        let code = load_capsule("code-agent").expect("Failed to load code-agent");
        
        // Different agents should have different capabilities
        assert_ne!(
            capsule_allows(&chat, "terminal"),
            capsule_allows(&code, "terminal")
        );
    }

    #[test]
    fn test_resource_limits_not_excessive() {
        let limits = load_limits().expect("Failed to load limits");
        
        for agent_type in ["default", "code-agent", "chat-agent", "task-agent"] {
            let agent_limits = &limits[agent_type];
            
            // No agent should use more than 90% CPU
            let cpu = agent_limits["cpu_percent"].as_u64().unwrap();
            assert!(cpu <= 90, "{} has excessive CPU: {}", agent_type, cpu);
            
            // No agent should use more than 8GB RAM
            let mem = agent_limits["memory_mb"].as_u64().unwrap();
            assert!(mem <= 8192, "{} has excessive memory: {}", agent_type, mem);
        }
    }
}
