//! Event Bus Module
//!
//! T181-T183: Event bus, workflow DAG engine, digest job queue
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

pub mod redis_streams;
pub mod workflow_engine;
pub mod digest_queue;

pub use redis_streams::RedisStreamsEventBus;
pub use workflow_engine::WorkflowDAGEngine;
pub use digest_queue::DigestJobQueue;

