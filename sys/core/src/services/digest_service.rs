//! Digest Service Orchestrator
//!
//! T184: Implement DigestService orchestrator
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use crate::db::{init_database, ConnectionPool};
use crate::error::Result;
use crate::services::digest::*;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Main digest service orchestrator that coordinates all pipeline stages
pub struct DigestService {
    db_path: PathBuf,
}

impl DigestService {
    /// Create a new digest service with a database path
    pub fn new(db_path: &Path) -> Self {
        Self {
            db_path: db_path.to_path_buf(),
        }
    }

    /// Execute the full digest pipeline for a source
    ///
    /// This orchestrates all 9 stages of the digest pipeline:
    /// 1. Intake (Discover)
    /// 2. Classifier (Languages, Licenses)
    /// 3. Graph Extract (Knowledge Graph)
    /// 4. Embeddings (Vector embeddings)
    /// 5. Env Synthesis (Dockerfiles, K8s)
    /// 6. Safety (SBOM, Security)
    /// 7. Runner (Build, Test, Demo)
    /// 8. Integrator (SDKs, Telemetry)
    /// 9. Registrar (Storage, Registry)
    ///
    /// # Arguments
    /// * `uri` - URI or path to the source
    /// * `source_type` - Type of source (Repository, File, Api, Document)
    ///
    /// # Returns
    /// Returns the UUID of the digested source
    ///
    /// # Errors
    /// Returns an error if any stage fails, with context about which stage failed
    pub async fn digest_source(
        &self,
        uri: &str,
        source_type: crate::db::repositories::DigestSourceType,
    ) -> Result<Uuid> {
        // Stage 1: Intake - Discover and register source
        let conn = init_database(&self.db_path)
            .map_err(|e| crate::error::NoaError::Database(crate::error::DatabaseError::ConnectionFailed {
                error: format!("Failed to initialize database at {}: {}", self.db_path.display(), e),
            }))?;
        let intake = IntakeService::new(conn);
        let source_id = intake.discover_source(uri, source_type).await
            .map_err(|e| crate::error::NoaError::Validation(crate::error::ValidationError::new(
                "intake",
                format!("Stage 1 (Intake) failed for source '{}': {}. Check URI validity and database access.", uri, e),
                "INTAKE_STAGE_FAILED",
            )))?;

        // Stage 2: Classifier - Identify languages and licenses
        let conn = init_database(&self.db_path)
            .map_err(|e| crate::error::NoaError::Database(crate::error::DatabaseError::ConnectionFailed {
                error: format!("Failed to initialize database for Stage 2: {}", e),
            }))?;
        let classifier = ClassifierService::new(conn);
        classifier.classify(&source_id).await
            .map_err(|e| crate::error::NoaError::Validation(crate::error::ValidationError::new(
                "classifier",
                format!("Stage 2 (Classifier) failed for source {}: {}. Check source accessibility.", source_id, e),
                "CLASSIFIER_STAGE_FAILED",
            )))?;

        // Stage 3: Graph Extract - Build knowledge graph
        let conn = init_database(&self.db_path)?;
        let graph_extract = GraphExtractService::new(conn);
        graph_extract.extract_graph(&source_id).await?;

        // Stage 4: Embeddings - Generate vector embeddings
        let conn = init_database(&self.db_path)?;
        let embeddings = EmbeddingsService::new(conn);
        embeddings.generate_embeddings(&source_id).await?;

        // Stage 5: Env Synthesis - Generate environment configs
        let conn = init_database(&self.db_path)?;
        let env_synthesis = EnvSynthesisService::new(conn);
        env_synthesis.synthesize(&source_id).await?;

        // Stage 6: Safety - Security scanning
        let conn = init_database(&self.db_path)?;
        let safety = SafetyService::new(conn);
        safety.scan(&source_id).await?;

        // Stage 7: Runner - Build, test, demo
        let conn = init_database(&self.db_path)?;
        let runner = RunnerService::new(conn);
        runner.run(&source_id).await?;

        // Stage 8: Integrator - SDKs and telemetry
        let conn = init_database(&self.db_path)?;
        let integrator = IntegratorService::new(conn);
        integrator.integrate(&source_id).await?;

        // Stage 9: Registrar - Store and register
        let conn = init_database(&self.db_path)?;
        let registrar = RegistrarService::new(conn);
        registrar.register(&source_id).await?;

        Ok(source_id)
    }
}

