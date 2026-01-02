use dioxus::prelude::*;
use keyboard_types::Key;

use crate::core::conversational_ai::{AIProvider, ConversationalAI, Conversation, ConversationContext, Message, MessageRole};

#[component]
pub fn ChatInterface() -> Element {
    let messages = use_signal(Vec::<Message>::new);
    let mut input_value = use_signal(String::new);
    let is_typing = use_signal(|| false);

    // Initialize AI provider with Ollama (local inference)
    let ai = use_signal(|| ConversationalAI::new(AIProvider::Local {
        endpoint: "http://localhost:11434".to_string(),
    }));

    // Note: we call `send_message_impl` directly from each handler since closures
    // cannot be reused across multiple RSX props without cloning.

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
                        if event.key() == Key::Enter {
                            send_message_impl(messages, input_value, is_typing);
                        }
                    }
                }

                button {
                    class: "send-button",
                    onclick: move |_| send_message_impl(messages, input_value, is_typing),
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
            class: message_class,

            div {
                class: "message-content",
                {message.content}
            }

            div {
                class: "message-time",
                {message.timestamp.format("%H:%M").to_string()}
            }
        }
    }
}

fn send_message_impl(
    mut messages: Signal<Vec<Message>>,
    mut input_value: Signal<String>,
    mut is_typing: Signal<bool>,
) {
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

    // Call Ollama for AI response
    spawn(async move {
        let client = reqwest::Client::new();
        let url = "http://localhost:11434/api/generate";
        
        let system_prompt = "You are Rust Lovable, an AI assistant that helps users build UIs using Rust and the Dioxus framework. \
                            IMPORTANT: This is a RUST application using DIOXUS, NOT React or JavaScript. \
                            Always provide code examples in Rust with Dioxus RSX syntax, not JSX or React. \
                            Dioxus uses the rsx! macro for component rendering, use_signal for state, and Rust idioms. \
                            Example Dioxus component: \
                            ```rust \
                            use dioxus::prelude::*; \
                            #[component] \
                            fn Counter() -> Element { \
                                let mut count = use_signal(|| 0); \
                                rsx! { \
                                    button { onclick: move |_| count += 1, \"Count: {count}\" } \
                                } \
                            } \
                            ``` \
                            Keep responses concise and actionable. Always use Rust/Dioxus syntax.";
        
        let prompt = format!("{}\n\nUser: {}\nAssistant:", system_prompt, user_message);
        
        let body = serde_json::json!({
            "model": "phi3:mini",
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.7,
                "num_predict": 300
            }
        });
        
        match client.post(url).json(&body).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    let ai_response = json.get("response")
                        .and_then(|r| r.as_str())
                        .unwrap_or("Sorry, I couldn't process that request.")
                        .to_string();
                    
                    let mut current_messages = messages.read().clone();
                    current_messages.push(Message::new(MessageRole::Assistant, ai_response));
                    messages.set(current_messages);
                } else {
                    let mut current_messages = messages.read().clone();
                    current_messages.push(Message::new(MessageRole::Assistant, "Error: Could not parse AI response.".to_string()));
                    messages.set(current_messages);
                }
            }
            Err(e) => {
                let mut current_messages = messages.read().clone();
                let error_msg = format!("Error connecting to AI: {}. Make sure Ollama is running with 'ollama serve'.", e);
                current_messages.push(Message::new(MessageRole::Assistant, error_msg));
                messages.set(current_messages);
            }
        }

        is_typing.set(false);
    });
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
