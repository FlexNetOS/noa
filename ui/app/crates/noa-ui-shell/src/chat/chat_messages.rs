//! Chat messages display component.

use dioxus::prelude::*;
use noa_api_client::ChatMessage;

/// Display a list of chat messages.
#[component]
pub fn ChatMessages(messages: Vec<ChatMessage>, is_loading: bool) -> Element {
    rsx! {
        div {
            class: "chat-messages flex-1 overflow-y-auto p-4 space-y-4",
            
            if messages.is_empty() {
                div {
                    class: "chat-empty text-center text-gray-500 py-8",
                    p { class: "text-lg", "Start a conversation" }
                    p { class: "text-sm", "Send a message to the AI assistant" }
                }
            }
            
            for message in messages.iter() {
                MessageBubble { message: message.clone() }
            }
            
            if is_loading {
                TypingIndicator {}
            }
        }
    }
}

/// Individual message bubble.
#[component]
fn MessageBubble(message: ChatMessage) -> Element {
    let is_user = message.role == "user";
    let is_system = message.role == "system";
    
    let container_class = if is_user {
        "flex justify-end"
    } else {
        "flex justify-start"
    };
    
    let bubble_class = if is_user {
        "chat-bubble chat-bubble-user bg-primary text-primary-content rounded-lg px-4 py-2 max-w-[80%]"
    } else if is_system {
        "chat-bubble chat-bubble-system bg-base-300 text-base-content rounded-lg px-4 py-2 max-w-[80%] italic"
    } else {
        "chat-bubble chat-bubble-assistant bg-base-200 text-base-content rounded-lg px-4 py-2 max-w-[80%]"
    };
    
    rsx! {
        div {
            class: container_class,
            
            div {
                class: bubble_class,
                
                // Role indicator for non-user messages
                if !is_user {
                    span {
                        class: "text-xs font-semibold opacity-70 block mb-1",
                        if is_system { "System" } else { "Assistant" }
                    }
                }
                
                // Message content with markdown support (simplified)
                div {
                    class: "message-content whitespace-pre-wrap",
                    {message.content.clone()}
                }
            }
        }
    }
}

/// Typing indicator for loading state.
#[component]
fn TypingIndicator() -> Element {
    rsx! {
        div {
            class: "flex justify-start",
            
            div {
                class: "chat-bubble bg-base-200 rounded-lg px-4 py-2",
                
                div {
                    class: "typing-indicator flex gap-1",
                    span { class: "w-2 h-2 bg-base-content rounded-full animate-bounce", style: "animation-delay: 0ms" }
                    span { class: "w-2 h-2 bg-base-content rounded-full animate-bounce", style: "animation-delay: 150ms" }
                    span { class: "w-2 h-2 bg-base-content rounded-full animate-bounce", style: "animation-delay: 300ms" }
                }
            }
        }
    }
}
