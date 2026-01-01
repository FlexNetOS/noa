use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
    pub variables: Vec<String>,
    pub description: String,
}

pub struct PromptEngineer {
    templates: HashMap<String, PromptTemplate>,
}

impl PromptEngineer {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        self.load_templates();
        Ok(())
    }
    
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub async fn engineer_prompt(&self, request: &crate::vibe_coding::VibeCodingRequest, context: &crate::vibe_coding::VibeCodingContext) -> Result<String> {
        // Engineer a well-structured prompt based on the request and context
        let mut prompt = format!(
            "Generate {} code for the following task: {}\n\n",
            context.language, request.task
        );
        
        if let Some(ctx) = &request.context {
            prompt.push_str(&format!("Context: {}\n\n", ctx));
        }
        
        if !request.constraints.is_empty() {
            prompt.push_str("Constraints:\n");
            for constraint in &request.constraints {
                prompt.push_str(&format!("- {}\n", constraint));
            }
            prompt.push_str("\n");
        }
        
        if !request.examples.is_empty() {
            prompt.push_str("Examples:\n");
            for (i, example) in request.examples.iter().enumerate() {
                prompt.push_str(&format!("{}: {}\n", i + 1, example));
            }
        }
        
        // Add coding style preferences
        if !context.coding_style.is_empty() {
            prompt.push_str(&format!("\nFollow {} coding style.", context.coding_style));
        }
        
        Ok(prompt)
    }
    
    fn load_templates(&mut self) {
        // Load predefined prompt templates
        self.templates.insert(
            "function".to_string(),
            PromptTemplate {
                name: "function".to_string(),
                template: "Create a function that {purpose}. The function should:\n\n{requirements}\n\nReturn type: {return_type}\nParameters: {parameters}".to_string(),
                variables: vec!["purpose".to_string(), "requirements".to_string(), "return_type".to_string(), "parameters".to_string()],
                description: "Template for generating functions".to_string(),
            }
        );
    }
}