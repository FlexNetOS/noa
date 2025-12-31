use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    pub sections: Vec<DocumentationSection>,
    pub format: DocumentationFormat,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSection {
    pub title: String,
    pub content: String,
    pub level: u8,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentationFormat {
    Markdown,
    Html,
    PlainText,
    RustDoc,
}

pub struct DocumentationGenerator {
    templates: HashMap<String, String>,
}

impl DocumentationGenerator {
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
    
    pub async fn generate_docs(&self, code: &[crate::vibe_coding::GeneratedCode]) -> Result<Documentation> {
        let mut sections = Vec::new();
        
        for code_snippet in code {
            sections.push(DocumentationSection {
                title: format!("{} - {}", code_snippet.file_name, code_snippet.purpose),
                content: self.generate_code_documentation(&code_snippet.code),
                level: 2,
                tags: vec![code_snippet.language.clone()],
            });
        }
        
        Ok(Documentation {
            sections,
            format: DocumentationFormat::Markdown,
            metadata: HashMap::new(),
        })
    }
    
    fn generate_code_documentation(&self, code: &str) -> String {
        let mut documentation = String::new();
        
        // Generate function documentation
        for line in code.lines() {
            if line.trim().starts_with("fn ") {
                let function_name = line.split('(').next().unwrap_or("").replace("fn ", "").trim().to_string();
                documentation.push_str(&format!("### Function: `{}`\n\n", function_name));
                documentation.push_str("Description: [Add function description]\n\n");
                documentation.push_str("Parameters:\n- [Add parameter descriptions]\n\n");
                documentation.push_str("Returns:\n- [Add return value description]\n\n");
            }
        }
        
        documentation
    }
    
    fn load_templates(&mut self) {
        // Load documentation templates
        self.templates.insert(
            "function".to_string(),
            "## Function: {name}\n\nDescription: {description}\n\nParameters:\n{parameters}\n\nReturns:\n{returns}\n\nExample:\n```rust\n{example}\n```".to_string()
        );
    }
}