//! Agent Integration Tests
//!
//! Tests for agent execution, coordination, and integration

use noa_core::agents::{
    CommanderChiefAgent, FileIOAgent, TerminalAgent, RAGAgent,
    base::BaseAgent,
    commander::{CommanderRequest, TaskPriority},
    file_io::{FileOperation, FileOperationResult},
    terminal::{TerminalCommand, TerminalResult},
    rag::RAGQuery,
};
use noa_core::error::Result;
use std::collections::HashMap;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_file_io_agent_read_write() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let agent = FileIOAgent::new();

    // Write operation
    let write_op = FileOperation::Write {
        path: test_file.to_string_lossy().to_string(),
        content: "Hello from integration test!".to_string(),
    };
    let result = agent.execute_operation(write_op)?;
    assert!(result.success, "Write operation should succeed");

    // Read operation
    let read_op = FileOperation::Read {
        path: test_file.to_string_lossy().to_string(),
    };
    let result = agent.execute_operation(read_op)?;
    assert!(result.success, "Read operation should succeed");
    assert_eq!(result.data.as_deref(), Some("Hello from integration test!"));

    Ok(())
}

#[test]
fn test_file_io_agent_list_directory() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    
    // Create some test files
    fs::write(temp_dir.path().join("file1.txt"), "content1")?;
    fs::write(temp_dir.path().join("file2.txt"), "content2")?;
    fs::create_dir(temp_dir.path().join("subdir"))?;

    let agent = FileIOAgent::new();
    let list_op = FileOperation::List {
        path: temp_dir.path().to_string_lossy().to_string(),
    };
    
    let result = agent.execute_operation(list_op)?;
    assert!(result.success, "List operation should succeed");
    
    let content = result.data.unwrap();
    assert!(content.contains("file1.txt"));
    assert!(content.contains("file2.txt"));
    assert!(content.contains("subdir/"));

    Ok(())
}

#[test]
fn test_file_io_agent_copy() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("source.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    fs::write(&source, "test content")?;
    
    let agent = FileIOAgent::new();
    let copy_op = FileOperation::Copy {
        from: source.to_string_lossy().to_string(),
        to: dest.to_string_lossy().to_string(),
    };
    
    let result = agent.execute_operation(copy_op)?;
    assert!(result.success, "Copy operation should succeed");
    
    // Verify destination exists and has correct content
    let content = fs::read_to_string(&dest)?;
    assert_eq!(content, "test content");

    Ok(())
}

#[test]
fn test_terminal_agent_echo() -> Result<()> {
    let agent = TerminalAgent::new();
    let cmd = TerminalCommand {
        command: "echo".to_string(),
        args: vec!["Hello".to_string(), "World".to_string()],
        working_dir: None,
        env: None,
        timeout_secs: Some(5),
    };
    
    let result = agent.execute_command(cmd)?;
    assert!(result.success, "Echo command should succeed");
    assert!(result.stdout.contains("Hello"));
    assert_eq!(result.exit_code, Some(0));

    Ok(())
}

#[test]
fn test_terminal_agent_whitelist() -> Result<()> {
    let agent = TerminalAgent::with_whitelist(vec!["echo".to_string()]);
    
    // Allowed command
    let result = agent.execute_simple("echo test")?;
    assert!(result.success);
    
    // Disallowed command
    let result = agent.execute_simple("dangerous_command")?;
    assert!(!result.success);
    assert!(result.error.is_some());
    assert!(result.error.unwrap().contains("not in whitelist"));

    Ok(())
}

#[test]
fn test_rag_agent_context_generation() -> Result<()> {
    let agent = RAGAgent::new();
    let context = vec![
        "Context item 1".to_string(),
        "Context item 2".to_string(),
        "Context item 3".to_string(),
    ];
    
    let prompt = agent.generate_with_context("What is the answer?", &context)?;
    assert!(prompt.contains("Context"));
    assert!(prompt.contains("Context item 1"));
    assert!(prompt.contains("What is the answer?"));

    Ok(())
}

#[test]
fn test_rag_agent_query_formatting() -> Result<()> {
    let agent = RAGAgent::new();
    let query = RAGQuery {
        query: "test query".to_string(),
        top_k: Some(10),
        filters: None,
        include_sources: true,
    };
    
    let formatted = agent.format_query(&query);
    assert!(formatted.contains("test query"));
    assert!(formatted.contains("10"));

    Ok(())
}

#[test]
fn test_commander_agent_file_task_decomposition() -> Result<()> {
    let commander = CommanderChiefAgent::new();
    let request = CommanderRequest {
        goal: "Read config.yaml and write to backup.yaml".to_string(),
        context: None,
        constraints: None,
    };
    
    let plan = commander.plan_execution(request)?;
    assert!(!plan.tasks.is_empty());
    assert!(plan.tasks.iter().any(|t| t.agent_name == "file-io"));
    assert_eq!(plan.goal, "Read config.yaml and write to backup.yaml");

    Ok(())
}

#[test]
fn test_commander_agent_multi_step_decomposition() -> Result<()> {
    let commander = CommanderChiefAgent::new();
    let request = CommanderRequest {
        goal: "Analyze the codebase for security issues".to_string(),
        context: None,
        constraints: None,
    };
    
    let plan = commander.plan_execution(request)?;
    assert!(!plan.tasks.is_empty());
    // Should involve RAG for context and model-selector for analysis
    assert!(plan.tasks.iter().any(|t| t.agent_name == "rag" || t.agent_name == "model-selector"));

    Ok(())
}

#[test]
fn test_commander_agent_execution_summary() -> Result<()> {
    let commander = CommanderChiefAgent::new();
    let request = CommanderRequest {
        goal: "Test goal".to_string(),
        context: None,
        constraints: None,
    };
    
    let plan = commander.plan_execution(request)?;
    let summary = commander.coordinate_execution(&plan)?;
    
    assert!(summary.contains("Test goal"));
    assert!(summary.contains("Total tasks"));

    Ok(())
}

#[test]
fn test_agent_base_trait_interface() -> Result<()> {
    let agents: Vec<Box<dyn BaseAgent>> = vec![
        Box::new(CommanderChiefAgent::new()),
        Box::new(FileIOAgent::new()),
        Box::new(TerminalAgent::new()),
        Box::new(RAGAgent::new()),
    ];
    
    for agent in agents {
        assert!(!agent.name().is_empty());
        assert!(!agent.description().is_empty());
        assert!(!agent.capabilities().is_empty());
        
        // Execute with a simple task
        let result = agent.execute("test task");
        assert!(result.is_ok());
    }

    Ok(())
}

#[test]
fn test_file_io_agent_json_api() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("api_test.txt");
    let agent = FileIOAgent::new();
    
    // Test JSON API via execute method
    let write_json = format!(
        r#"{{"op": "write", "path": "{}", "content": "JSON API test"}}"#,
        test_file.to_string_lossy().replace("\\", "\\\\")
    );
    
    let result_str = agent.execute(&write_json)?;
    let result: FileOperationResult = serde_json::from_str(&result_str)?;
    assert!(result.success);

    Ok(())
}

#[test]
fn test_terminal_agent_json_api() -> Result<()> {
    let agent = TerminalAgent::new();
    
    let cmd_json = r#"{"command": "echo", "args": ["test"], "timeout_secs": 5}"#;
    let result_str = agent.execute(cmd_json)?;
    let result: TerminalResult = serde_json::from_str(&result_str)?;
    
    assert!(result.success);
    assert!(result.stdout.contains("test"));

    Ok(())
}

#[test]
fn test_commander_agent_json_api() -> Result<()> {
    let commander = CommanderChiefAgent::new();
    
    let request_json = r#"{"goal": "Test JSON API", "context": null, "constraints": null}"#;
    let result = commander.execute(request_json)?;
    
    assert!(result.contains("Test JSON API"));
    assert!(result.contains("Total tasks"));

    Ok(())
}

#[test]
fn test_agent_error_handling() -> Result<()> {
    let agent = FileIOAgent::new();
    
    // Try to read non-existent file
    let read_op = FileOperation::Read {
        path: "/nonexistent/file.txt".to_string(),
    };
    
    let result = agent.execute_operation(read_op)?;
    assert!(!result.success);
    assert!(result.error.is_some());

    Ok(())
}

#[test]
fn test_file_io_agent_with_base_path() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let agent = FileIOAgent::with_base_path(temp_dir.path().to_path_buf());
    
    // Write using relative path (should be resolved to base_path)
    let write_op = FileOperation::Write {
        path: "relative_test.txt".to_string(),
        content: "Base path test".to_string(),
    };
    
    let result = agent.execute_operation(write_op)?;
    assert!(result.success);
    
    // Verify file was created in base path
    let expected_path = temp_dir.path().join("relative_test.txt");
    assert!(expected_path.exists());

    Ok(())
}
