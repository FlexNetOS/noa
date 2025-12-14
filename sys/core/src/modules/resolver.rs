use crate::modules::registry::ModuleRegistry;
use crate::modules::types::ModuleMetadata;
use crate::error::{Result, NoaError};
use std::collections::{HashMap, HashSet};

/// Resolve dependencies depth-first and return an ordered list (parents before children).
pub fn resolve_dependencies(registry: &ModuleRegistry, root: &str) -> Result<Vec<ModuleMetadata>> {
    let mut ordered = Vec::new();
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();

    fn dfs(
        registry: &ModuleRegistry,
        name: &str,
        ordered: &mut Vec<ModuleMetadata>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) -> Result<()> {
        if visited.contains(name) {
            return Ok(());
        }
        if !visiting.insert(name.to_string()) {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "module.dependencies",
                format!("cycle detected at {}", name),
                "CYCLE",
            )));
        }

        let Some(meta) = registry.find_by_name(name)? else {
            return Err(NoaError::NotFound { resource: "module".into(), id: name.into() });
        };

        for dep in &meta.dependencies {
            dfs(registry, &dep.name, ordered, visiting, visited)?;
        }
        visiting.remove(name);
        visited.insert(name.to_string());
        ordered.push(meta);
        Ok(())
    }

    dfs(registry, root, &mut ordered, &mut visiting, &mut visited)?;
    Ok(ordered)
}

/// Render a simple ASCII tree of dependencies.
pub fn render_tree(registry: &ModuleRegistry, root: &str) -> Result<String> {
    let mut output = String::new();
    let mut seen = HashSet::new();
    render_node(registry, root, "", &mut seen, &mut output)?;
    Ok(output)
}

fn render_node(
    registry: &ModuleRegistry,
    name: &str,
    prefix: &str,
    seen: &mut HashSet<String>,
    output: &mut String,
) -> Result<()> {
    if !seen.insert(name.to_string()) {
        output.push_str(&format!("{}- {} (cycle)\n", prefix, name));
        return Ok(());
    }

    let Some(meta) = registry.find_by_name(name)? else {
        output.push_str(&format!("{}- {} (missing)\n", prefix, name));
        return Ok(());
    };

    output.push_str(&format!(
        "{}- {} v{} [{}]\n",
        prefix,
        meta.name,
        meta.version,
        meta.module_type.as_str()
    ));

    let next_prefix = format!("{}  ", prefix);
    for dep in &meta.dependencies {
        render_node(registry, &dep.name, &next_prefix, seen, output)?;
    }
    Ok(())
}
