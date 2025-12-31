use dioxus::prelude::*;
use std::collections::VecDeque;

use crate::core::conversational_ai::{Conversation, Message, MessageRole, ConversationalAI, AIProvider};

#[component]
pub fn ChatInterface() -> Element {
    let messages = use_signal(Vec::<Message>::new);
    let input_value = use_signal(String::new);
    let is_typing = use_signal(|| false);
    
    // Initialize AI provider (in a real app, this would be configurable)
    let ai = ConversationalAI::new(AIProvider::Local { 
        endpoint: "http://localhost:8080/ai".to_string() 
    });
    
    let send_message = move |_| {
        let user_message = input_value.read().clone();
        if user_message.is_empty() {
            return;
        }
        
        // Add user message
        let mut current_messages = messages.read().clone();
        current_messages.push(Message::new(MessageRole::User, user_message.clone()));
        messages.set(current_messages);
        
        // Clear input
        input_value.set(String::new());
        
        // Show typing indicator
        is_typing.set(true);
        
        // Simulate AI response (in real app, this would call the AI provider)
        spawn(async move {
            // Simulate delay
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            
            let mut current_messages = messages.read().clone();
            let ai_response = format!("I understand you want to: {}", user_message);
            current_messages.push(Message::new(MessageRole::Assistant, ai_response));
            messages.set(current_messages);
            
            is_typing.set(false);
        });
    };
    
    rsx! {
        div {
            class: "chat-interface",
            
            // Chat header
            div {
                class: "chat-header",
                h3 { "AI Assistant" }
                div {
                    class: "ai-status",
                    if *is_typing.read() {
                        span { class: "typing-indicator", "AI is thinking..." }
                    } else {
                        span { class: "status-ready", "Ready" }
                    }
                }
            }
            
            // Messages container
            div {
                class: "messages-container",
                
                for message in messages.read().iter() {
                    MessageBubble { message: message.clone() }
                }
                
                if *is_typing.read() {
                    div {
                        class: "typing-bubble ai-message",
                        div {
                            class: "typing-dots",
                            span { "." }
                            span { "." }
                            span { "." }
                        }
                    }
                }
            }
            
            // Input area
            div {
                class: "chat-input-area",
                
                textarea {
                    class: "message-input",
                    placeholder: "Describe the UI changes you want...",
                    value: "{input_value.read()}",
                    oninput: move |event| {
                        input_value.set(event.value().to_string());
                    },
                    onkeyup: move |event| {
                        if event.key() == "Enter" && !event.shift_key() {
                            send_message(());
                        }
                    }
                }
                
                button {
                    class: "send-button",
                    onclick: send_message,
                    disabled: input_value.read().is_empty(),
                    "Send"
                }
            }
        }
    }
}

#[component]
pub fn MessageBubble(message: Message) -> Element {
    let is_user = message.role == MessageRole::User;
    let message_class = if is_user {
        "message-bubble user-message"
    } else {
        "message-bubble ai-message"
    };
    
    rsx! {
        div {
            class: "{message_class}",
            
            div {
                class: "message-content",
                "{message.content}"
            }
            
            div {
                class: "message-time",
                "{message.timestamp.format("%H:%M")}"
            }
        }
    }
}

// Quick action buttons for common UI tasks
#[component]
pub fn QuickActions() -> Element {
    let actions = vec![
        ("Add Button", "Create a new button component"),
        ("Add Form", "Create a new form with inputs"),
        ("Add Card", "Create a card component"),
        ("Add Modal", "Create a modal dialog"),
    ];
    
    rsx! {
        div {
            class: "quick-actions",
            h4 { "Quick Actions" }
            
            for (title, description) in actions {
                button {
                    class: "quick-action-button",
                    onclick: move |_| {
                        // Send quick action as message
                    },
                    
                    div {
                        class: "action-title",
                        "{title}"
                    }
                    
                    div {
                        class: "action-description",
                        "{description}"
                    }
                }
            }
        }
    }
}