//! Ask Command for Inference
//!
//! T123: Implement `noa ask` command for inference
//! US2: CLI command for model inference

use crate::db::init_database;
use crate::error::Result;
use crate::neural::inference::InferenceRequest;
use crate::services::NeuralService;
use clap::Args;
use std::path::PathBuf;
use uuid::Uuid;

/// Ask command arguments
#[derive(Args)]
pub struct AskArgs {
    /// Prompt to send to the model
    pub prompt: String,
    /// Model ID to use (optional, will select automatically if not provided)
    #[arg(short, long)]
    pub model: Option<String>,
    /// Context ID for conversation continuity
    #[arg(short, long)]
    pub context: Option<String>,
    /// Temperature for generation
    #[arg(short, long)]
    pub temperature: Option<f32>,
    /// Maximum tokens to generate
    #[arg(short, long)]
    pub max_tokens: Option<usize>,
    /// Enable streaming output
    #[arg(short, long)]
    pub stream: bool,
}

/// Execute ask command
pub async fn execute(args: AskArgs, noa_root: Option<String>) -> Result<()> {
    let db_path = noa_root
        .map(|r| PathBuf::from(r).join("data").join("noa.db"))
        .unwrap_or_else(|| PathBuf::from("data").join("noa.db"));

    let conn = init_database(&db_path)?;
    let neural_service = NeuralService::new(conn);

    // Determine model ID
    let model_id = if let Some(model_str) = args.model {
        // Use specified model
        Uuid::parse_str(&model_str)
            .map_err(|_| {
                crate::error::NoaError::Validation(crate::error::ValidationError::new(
                    "model",
                    "Invalid UUID format",
                    "INVALID_UUID",
                ))
            })?
            .to_string()
    } else {
        // Auto-select model (use first loaded model or first available)
        let loaded = neural_service.get_loaded_models()?;
        if let Some(model) = loaded.first() {
            model.id.to_string()
        } else {
            let all = neural_service.list_models()?;
            if let Some(model) = all.first() {
                // Try to load it
                neural_service.load_model(&model.id).await?;
                model.id.to_string()
            } else {
                return Err(crate::error::NoaError::NotFound {
                    resource: "Model".to_string(),
                    id: "none".to_string(),
                });
            }
        }
    };

    // Parse context ID if provided
    let context_id = args.context.map(|c| Uuid::parse_str(&c)).transpose().map_err(|_| {
        crate::error::NoaError::Validation(crate::error::ValidationError::new(
            "context",
            "Invalid UUID format",
            "INVALID_UUID",
        ))
    })?;

    let engine = neural_service.inference_engine();

    let request = InferenceRequest {
        model_id,
        prompt: args.prompt,
        context_id,
        temperature: args.temperature,
        top_p: Some(0.9),
        top_k: Some(40),
        max_tokens: args.max_tokens,
        stream: args.stream,
    };

    if args.stream {
        // Stream response
        let stream = engine.infer_stream(request).await?;
        use tokio_stream::StreamExt;
        tokio::pin!(stream);
        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    print!("{}", chunk.content);
                    if chunk.done {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("\nError: {}", e);
                    break;
                }
            }
        }
        println!();
    } else {
        // Non-streaming response
        let response = engine.infer(request).await?;
        println!("{}", response.content);
        if let Some(ctx_id) = response.context_id {
            println!("\n[Context ID: {}]", ctx_id);
        }
    }

    Ok(())
}
