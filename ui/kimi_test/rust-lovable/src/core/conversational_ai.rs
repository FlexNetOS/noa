use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<serde_json::Value>,
}

impl Message {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role,
            content,
            timestamp: chrono::Utc::now(),
            metadata: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub messages: VecDeque<Message>,
    pub context: ConversationContext,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub project_id: Option<String>,
    pub current_file: Option<String>,
    pub ui_state: Option<UIState>,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIState {
    pub selected_component: Option<String>,
    pub view_mode: ViewMode,
    pub theme: String,
    pub platform_target: PlatformTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViewMode {
    Design,
    Code,
    Split,
    Preview,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlatformTarget {
    Web,
    Desktop,
    Mobile,
    Universal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub code_style: CodeStyle,
    pub ui_framework: UIFramework,
    pub color_scheme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeStyle {
    Functional,
    ClassBased,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIFramework {
    Dioxus,
    React,
    Vue,
    Svelte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIChangeRequest {
    pub description: String,
    pub target_component: Option<String>,
    pub change_type: UIChangeType,
    pub platform_specific: Option<PlatformSpecific>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIChangeType {
    CreateComponent,
    ModifyComponent,
    DeleteComponent,
    RestructureLayout,
    ChangeStyling,
    AddInteraction,
    UpdateContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSpecific {
    pub platform: PlatformTarget,
    pub adaptations: Vec<UIAdaptation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAdaptation {
    pub property: String,
    pub value: String,
    pub condition: Option<String>,
}

pub struct ConversationalAI {
    provider: AIProvider,
    context_window: usize,
}

#[derive(Debug, Clone)]
pub enum AIProvider {
    OpenAI { api_key: String, model: String },
    Anthropic { api_key: String, model: String },
    Groq { api_key: String, model: String },
    Local { endpoint: String },
}

impl ConversationalAI {
    pub fn new(provider: AIProvider) -> Self {
        Self {
            provider,
            context_window: 4096,
        }
    }

    pub async fn process_message(&self, conversation: &mut Conversation, user_input: String) -> Result<Message, AIError> {
        // Add user message
        let user_message = Message::new(MessageRole::User, user_input.clone());
        conversation.messages.push_back(user_message.clone());

        // Prepare context for AI
        let context = self.prepare_context(conversation);
        
        // Call AI provider
        let response = self.call_ai_provider(&context, &user_input).await?;
        
        // Add assistant message
        let assistant_message = Message::new(MessageRole::Assistant, response);
        conversation.messages.push_back(assistant_message.clone());
        
        // Update conversation timestamp
        conversation.updated_at = chrono::Utc::now();
        
        // Trim messages if context window is exceeded
        self.trim_context(conversation);
        
        Ok(assistant_message)
    }

    fn prepare_context(&self, conversation: &Conversation) -> String {
        let mut context = String::new();
        
        // Add system context
        context.push_str("You are Rust Lovable, an AI assistant that helps users build UIs through conversation. ");
        context.push_str("You can understand UI change requests and generate appropriate code modifications. ");
        context.push_str("Always respond with clear, actionable instructions that can be parsed into UI change requests.\n\n");
        
        // Add conversation context
        for message in conversation.messages.iter().rev().take(10) {
            match message.role {
                MessageRole::User => context.push_str(&format!("User: {}\n", message.content)),
                MessageRole::Assistant => context.push_str(&format!("Assistant: {}\n", message.content)),
                MessageRole::System => context.push_str(&format!("System: {}\n", message.content)),
            }
        }
        
        context
    }

    async fn call_ai_provider(&self, context: &str, user_input: &str) -> Result<String, AIError> {
        match &self.provider {
            AIProvider::OpenAI { api_key, model } => {
                // Implementation for OpenAI API
                Ok("OpenAI response would go here".to_string())
            }
            AIProvider::Anthropic { api_key, model } => {
                // Implementation for Anthropic API
                Ok("Anthropic response would go here".to_string())
            }
            AIProvider::Groq { api_key, model } => {
                // Implementation for Groq API
                Ok("Groq response would go here".to_string())
            }
            AIProvider::Local { endpoint } => {
                // Implementation for local AI endpoint
                Ok("Local AI response would go here".to_string())
            }
        }
    }

    fn trim_context(&self, conversation: &mut Conversation) {
        while conversation.messages.len() > self.context_window {
            conversation.messages.pop_front();
        }
    }

    pub fn parse_ui_request(&self, message: &str) -> Result<UIChangeRequest, AIError> {
        // Parse natural language into structured UI change request
        // This would use more sophisticated NLP in a real implementation
        
        let change_type = if message.contains("create") || message.contains("add") {
            UIChangeType::CreateComponent
        } else if message.contains("change") || message.contains("modify") || message.contains("update") {
            UIChangeType::ModifyComponent
        } else if message.contains("delete") || message.contains("remove") {
            UIChangeType::DeleteComponent
        } else if message.contains("layout") || message.contains("structure") {
            UIChangeType::RestructureLayout
        } else if message.contains("style") || message.contains("color") || message.contains("look") {
            UIChangeType::ChangeStyling
        } else if message.contains("click") || message.contains("interaction") || message.contains("behavior") {
            UIChangeType::AddInteraction
        } else {
            UIChangeType::UpdateContent
        };
        
        Ok(UIChangeRequest {
            description: message.to_string(),
            target_component: None, // Extract from message
            change_type,
            platform_specific: None,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Parsing error: {0}")]
    Parsing(String),
    
    #[error("Context window exceeded")]
    ContextExceeded,
}