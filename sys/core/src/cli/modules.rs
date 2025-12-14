use crate::modules::registry::ModuleRegistry;
use crate::modules::resolver::{render_tree, resolve_dependencies};
use crate::modules::cas::ContentAddressableStore;
use crate::modules::verify::verify_hash;
use crate::error::Result;
use std::path::PathBuf;

pub enum ModuleCmd {
    List,
    Info { name: String },
    Verify { name: String },
    Deps { name: String },
}

pub async fn execute(cmd: ModuleCmd, noa_root: Option<String>) -> Result<()> {
    let root = PathBuf::from(noa_root.unwrap_or_else(|| ".".into()));
    let registry_path = root.join("data/modules/registry/registry.db");
    let cas_path = root.join("data/modules/cas");

    let registry = ModuleRegistry::new(&registry_path)?;
    let cas = ContentAddressableStore::new(&cas_path)?;

    match cmd {
        ModuleCmd::List => {
            let modules = registry.list_modules()?;
            println!("{:<24} {:<12} {:<10} {}", "Name", "Type", "Version", "Hash");
            for m in modules {
                println!(
                    "{:<24} {:<12} {:<10} {}",
                    m.name,
                    m.module_type.as_str(),
                    m.version,
                    &m.hash[..std::cmp::min(12, m.hash.len())]
                );
            }
        }
        ModuleCmd::Info { name } => {
            if let Some(meta) = registry.find_by_name(&name)? {
                println!("{}", serde_json::to_string_pretty(&meta)?);
            } else {
                println!("Module not found: {}", name);
            }
        }
        ModuleCmd::Verify { name } => {
            if let Some(meta) = registry.find_by_name(&name)? {
                if cas.exists(&meta.hash) {
                    if let Some(path) = meta.path.clone() {
                        let ok = verify_hash(&path, &meta.hash)?;
                        println!("{}: CAS object exists, disk verification={}", name, ok);
                    } else {
                        println!("{}: CAS object exists (no on-disk path provided)", name);
                    }
                } else {
                    println!("{}: missing CAS object {}", name, meta.hash);
                }
            } else {
                println!("Module not found: {}", name);
            }
        }
        ModuleCmd::Deps { name } => {
            let tree = render_tree(&registry, &name)?;
            println!("{}", tree);
        }
    }

    Ok(())
}
