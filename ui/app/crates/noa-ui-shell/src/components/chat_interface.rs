//! Chat interface component
//!
//! Provides the main conversational interface for interacting with the AI.

use dioxus::prelude::*;

/// Chat interface component for AI conversations
pub struct ChatInterface {
    /// Current message being composed
    pub current_message: String,
    /// Conversation history
    pub messages: Vec<ChatMessage>,
    /// Whether AI is currently processing
    pub is_processing: bool,
}

/// A single chat message
#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessage {
    /// Unique message ID
    pub id: String,
    /// Message content
    pub content: String,
    /// Message role (user, assistant, system)
    pub role: MessageRole,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Role of a message sender
#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl ChatInterface {
    /// Create a new chat interface
    pub fn new() -> Self {
        Self {
            current_message: String::new(),
            messages: Vec::new(),
            is_processing: false,
        }
    }

    /// Add a message to the conversation
    pub fn add_message(&mut self, content: String, role: MessageRole) {
        let message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            role,
            timestamp: chrono::Utc::now(),
        };
        self.messages.push(message);
    }

    /// Clear the conversation
    pub fn clear(&mut self) {
        self.messages.clear();
        self.current_message.clear();
    }
}

impl Default for ChatInterface {
    fn default() -> Self {
        Self::new()
    }
}

/// Chat interface Dioxus component
#[component]
pub fn ChatInterfaceComponent() -> Element {
    let mut messages = use_signal(Vec::<ChatMessage>::new);
    let mut input_text = use_signal(String::new);
    let is_processing = use_signal(|| false);

    rsx! {
        div {
            class: "chat-interface",

            // Message list
            div {
                class: "chat-messages",
                for msg in messages.read().iter() {
                    div {
                        class: "chat-message",
                        key: "{msg.id}",
                        p { "{msg.content}" }
                    }
                }
            }

            // Input area
            div {
                class: "chat-input",
                input {
                    r#type: "text",
                    placeholder: "Type your message...",
                    value: "{input_text}",
                    oninput: move |e| input_text.set(e.value().clone()),
                }
                button {
                    disabled: is_processing(),
                    onclick: move |_| {
                        let text = input_text.read().clone();
                        if !text.is_empty() {
                            messages.write().push(ChatMessage {
                                id: uuid::Uuid::new_v4().to_string(),
                                content: text,
                                role: MessageRole::User,
                                timestamp: chrono::Utc::now(),
                            });
                            input_text.set(String::new());
                        }
                    },
                    "Send"
                }
            }
        }
    }
}
