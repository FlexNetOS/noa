//! Model Management CLI Commands
//!
//! T120-T124: Implement model management CLI commands
//! US2: CLI commands for model management

use crate::db::init_database;
use crate::db::repositories::{ModelRepository, Model, ModelStatus};
use crate::error::Result;
use crate::services::{NeuralService, ModelDownloadService};
use crate::neural::benchmark::{ModelBenchmark, BenchmarkConfig};
use clap::{Args, Subcommand};
use std::path::PathBuf;
use uuid::Uuid;

/// Model management commands
#[derive(Subcommand)]
pub enum ModelCommands {
    /// List all registered models
    List,
    /// Download a model
    Download {
        /// Model name
        name: String,
        /// Download URL
        url: String,
        /// Output path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Verify a model file
    Verify {
        /// Model file path
        path: PathBuf,
    },
    /// Benchmark a model
    Benchmark {
        /// Model ID
        model_id: String,
        /// Number of iterations
        #[arg(short, long, default_value = "10")]
        iterations: usize,
    },
}

/// Model command arguments
#[derive(Args)]
pub struct ModelArgs {
    #[command(subcommand)]
    pub command: ModelCommands,
}

/// Execute model command
pub async fn execute(args: ModelArgs, noa_root: Option<String>) -> Result<()> {
    let db_path = noa_root
        .map(|r| PathBuf::from(r).join("data").join("noa.db"))
        .unwrap_or_else(|| PathBuf::from("data").join("noa.db"));

    let conn = init_database(&db_path)?;
    let neural_service = NeuralService::new(conn);
    let download_service = ModelDownloadService::new();

    match args.command {
        ModelCommands::List => {
            let models = neural_service.list_models()?;
            println!("Registered Models:");
            println!("{:-<80}", "");
            for model in models {
                println!("ID: {}", model.id);
                println!("Name: {}", model.name);
                println!("Type: {:?}", model.model_type);
                println!("Provider: {}", model.provider);
                println!("Status: {:?}", model.status);
                if let Some(path) = &model.path {
                    println!("Path: {}", path);
                }
                println!("{:-<80}", "");
            }
            Ok(())
        }
        ModelCommands::Download { name, url, output } => {
            let output_path = output.unwrap_or_else(|| {
                PathBuf::from("models").join(format!("{}.gguf", name))
            });

            println!("Starting download: {} -> {:?}", url, output_path);
            let download_id = download_service.download_model(name, url, output_path).await?;
            println!("Download started with ID: {}", download_id);
            println!("Use 'noa models status {}' to check progress", download_id);
            Ok(())
        }
        ModelCommands::Verify { path } => {
            use crate::neural::model_loader::ModelLoader;
            let loader = ModelLoader::new();
            let valid = loader.validate_gguf(&path).await?;
            if valid {
                println!("✓ Model file is valid GGUF format");
                if let Ok(metadata) = loader.get_metadata(&path).await {
                    println!("Model: {}", metadata.name);
                    println!("Size: {} bytes", metadata.size_bytes);
                    println!("Format: {}", metadata.format);
                    if let Some(quant) = &metadata.quantization {
                        println!("Quantization: {}", quant);
                    }
                }
            } else {
                println!("✗ Invalid model file");
            }
            Ok(())
        }
        ModelCommands::Benchmark { model_id, iterations } => {
            let model_uuid = Uuid::parse_str(&model_id)
                .map_err(|_| crate::error::NoaError::Validation(
                    crate::error::ValidationError::new(
                        "model_id",
                        "Invalid UUID format",
                        "INVALID_UUID",
                    ),
                ))?;

            let model = neural_service.get_model(&model_uuid)?
                .ok_or_else(|| crate::error::NoaError::NotFound {
                    resource: "Model".to_string(),
                    id: model_id,
                })?;

            println!("Benchmarking model: {}", model.name);
            println!("Iterations: {}", iterations);

            // Ensure model is loaded
            if model.status != ModelStatus::Loaded {
                println!("Loading model...");
                neural_service.load_model(&model_uuid).await?;
            }

            let engine = neural_service.inference_engine();
            let benchmark = ModelBenchmark::new();

            let config = BenchmarkConfig {
                model_id: model_id.clone(),
                test_prompts: vec![
                    "Hello, how are you?".to_string(),
                    "What is the capital of France?".to_string(),
                    "Explain quantum computing in simple terms.".to_string(),
                ],
                iterations,
                warmup_iterations: 2,
            };

            let results = benchmark.benchmark(&engine, config).await?;

            println!("\nBenchmark Results:");
            println!("{:-<80}", "");
            println!("Average Latency: {:.2} ms", results.average_latency_ms);
            println!("Min Latency: {:.2} ms", results.min_latency_ms);
            println!("Max Latency: {:.2} ms", results.max_latency_ms);
            println!("Tokens/Second: {:.2}", results.tokens_per_second);
            println!("Successful: {}/{}", results.successful_iterations, results.total_iterations);
            if results.failed_iterations > 0 {
                println!("Failed: {}", results.failed_iterations);
            }

            Ok(())
        }
    }
}

