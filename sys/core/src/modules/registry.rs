use crate::error::{DatabaseError, NoaError, Result};
use crate::modules::types::{ModuleDependency, ModuleMetadata, ModuleType};
use rusqlite::{params, Connection};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ModuleRegistry {
    conn: Connection,
    db_path: PathBuf,
}

impl ModuleRegistry {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let db_path = db_path.as_ref().to_path_buf();
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&db_path).map_err(|e| {
            NoaError::Database(DatabaseError::ConnectionFailed(e.to_string()))
        })?;
        Self::apply_pragmas(&conn)?;
        Ok(Self { conn, db_path })
    }

    fn apply_pragmas(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;
        "#,
        )
        .map_err(|e| {
            NoaError::Database(DatabaseError::QueryFailed {
                query: "PRAGMA".into(),
                error: e.to_string(),
            })
        })
    }

    pub fn register(&self, meta: &ModuleMetadata) -> Result<()> {
        let ts = now();
        self.conn
            .execute(
                "INSERT OR IGNORE INTO modules (id, name, module_type, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![meta.id, meta.name, meta.module_type.as_str(), ts, ts],
            )
            .map_err(to_db_err("insert module"))?;

        self.conn
            .execute(
                "UPDATE modules SET module_type = ?1, updated_at = ?2 WHERE name = ?3",
                params![meta.module_type.as_str(), ts, meta.name],
            )
            .map_err(to_db_err("update module"))?;

        self.conn
            .execute(
                "INSERT INTO module_versions (module_id, version, hash, metadata, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![meta.id, meta.version, meta.hash, serde_json::to_string(meta).unwrap_or_default(), ts],
            )
            .map_err(to_db_err("insert version"))?;

        let version_id = self.conn.last_insert_rowid();

        for dep in &meta.dependencies {
            self.conn
                .execute(
                    "INSERT INTO module_dependencies (module_version_id, dependency_name, version_constraint, optional) VALUES (?1, ?2, ?3, ?4)",
                    params![version_id, dep.name, dep.version_constraint, dep.optional],
                )
                .map_err(to_db_err("insert dependency"))?;
        }

        for cap in &meta.capabilities {
            self.conn
                .execute(
                    "INSERT INTO module_capabilities (module_version_id, capability) VALUES (?1, ?2)",
                    params![version_id, cap],
                )
                .map_err(to_db_err("insert capability"))?;
        }

        Ok(())
    }

    pub fn list_modules(&self) -> Result<Vec<ModuleMetadata>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT m.id, m.name, m.module_type,
                   COALESCE(MAX(v.version), '0.0.0') as version,
                   COALESCE(MAX(v.hash), '') as hash
            FROM modules m
            LEFT JOIN module_versions v ON v.module_id = m.id
            GROUP BY m.id
        "#,
        ).map_err(to_db_err("select modules"))?;

        let rows = stmt
            .query_map([], |row| {
                let module_type_str: String = row.get(2)?;
                Ok(ModuleMetadata {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    module_type: parse_module_type(&module_type_str),
                    version: row.get(3).unwrap_or_else(|_| "0.0.0".to_string()),
                    hash: row.get(4).unwrap_or_else(|_| "".to_string()),
                    capabilities: Vec::new(),
                    dependencies: Vec::new(),
                    path: None,
                })
            })
            .map_err(to_db_err("map modules"))?;

        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    pub fn find_by_name(&self, name: &str) -> Result<Option<ModuleMetadata>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT m.id, m.name, m.module_type,
                   v.id as version_id, v.version, v.hash
            FROM modules m
            LEFT JOIN module_versions v ON v.module_id = m.id
            WHERE m.name = ?1
            ORDER BY v.id DESC
            LIMIT 1
        "#,
        ).map_err(to_db_err("select module by name"))?;

        let row = stmt.query_row(params![name], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        });

        let (id, name, module_type_str, version_id, version, hash) = match row {
            Ok(tuple) => tuple,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(to_db_err("query module")(e)),
        };

        let deps = self.dependencies_for_version(version_id)?;
        let caps = self.capabilities_for_version(version_id)?;
        let meta = ModuleMetadata {
            id,
            name,
            module_type: parse_module_type(&module_type_str),
            version,
            hash,
            capabilities: caps,
            dependencies: deps,
            path: None,
        };
        Ok(Some(meta))
    }

    fn dependencies_for_version(&self, version_id: i64) -> Result<Vec<ModuleDependency>> {
        let mut stmt = self.conn.prepare(
            "SELECT dependency_name, version_constraint, optional FROM module_dependencies WHERE module_version_id = ?1",
        ).map_err(to_db_err("select dependencies"))?;
        let mut rows = stmt.query(params![version_id]).map_err(to_db_err("query dependencies"))?;
        let mut deps = Vec::new();
        while let Some(row) = rows.next().map_err(to_db_err("dep row"))? {
            deps.push(ModuleDependency {
                name: row.get(0)?,
                version_constraint: row.get(1)?,
                optional: row.get::<_, i64>(2)? != 0,
            });
        }
        Ok(deps)
    }

    fn capabilities_for_version(&self, version_id: i64) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT capability FROM module_capabilities WHERE module_version_id = ?1",
        ).map_err(to_db_err("select capabilities"))?;
        let mut rows = stmt.query(params![version_id]).map_err(to_db_err("query capabilities"))?;
        let mut caps = Vec::new();
        while let Some(row) = rows.next().map_err(to_db_err("cap row"))? {
            caps.push(row.get(0)?);
        }
        Ok(caps)
    }

    pub fn path(&self) -> &Path {
        &self.db_path
    }
}

fn parse_module_type(value: &str) -> ModuleType {
    match value {
        "binary" => ModuleType::Binary,
        "package" => ModuleType::Package,
        "library" => ModuleType::Library,
        "tool" => ModuleType::Tool,
        "service" => ModuleType::Service,
        "agent" => ModuleType::Agent,
        "microkernel" => ModuleType::Microkernel,
        _ => ModuleType::Library,
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn to_db_err(context: &'static str) -> impl Fn(rusqlite::Error) -> NoaError {
    move |err| NoaError::Database(DatabaseError::QueryFailed {
        query: context.into(),
        error: err.to_string(),
    })
}
