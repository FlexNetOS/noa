//! Chat hooks for API integration.

use dioxus::prelude::*;
use noa_api_client::{Client, ChatRequest, ChatResponse, ChatMessage, ProvidersResponse, Provider};
use std::sync::Arc;

/// API client context.
pub static API_CLIENT: GlobalSignal<Option<Arc<Client>>> = Signal::global(|| None);

/// Initialize the API client.
pub fn use_api_client() -> Arc<Client> {
    let client = API_CLIENT.read();
    client.clone().unwrap_or_else(|| {
        let new_client = Arc::new(Client::default());
        // Store for future use
        *API_CLIENT.write() = Some(new_client.clone());
        new_client
    })
}

/// Chat state for a conversation.
#[derive(Clone, Debug, Default)]
pub struct ChatState {
    pub messages: Vec<ChatMessage>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub current_provider: Option<String>,
    pub current_model: Option<String>,
}

/// Use chat state hook.
pub fn use_chat_state() -> Signal<ChatState> {
    use_signal(ChatState::default)
}

/// Send a chat message and get a response.
pub async fn send_message(
    client: Arc<Client>,
    message: String,
    history: Vec<ChatMessage>,
    provider: Option<String>,
    model: Option<String>,
) -> Result<ChatResponse, String> {
    let request = ChatRequest {
        message,
        provider,
        model,
        history: if history.is_empty() { None } else { Some(history) },
        stream: false,
    };

    client.chat(request).await.map_err(|e| e.to_string())
}

/// Fetch available providers.
pub async fn fetch_providers(client: Arc<Client>) -> Result<Vec<Provider>, String> {
    client
        .list_providers()
        .await
        .map(|r| r.providers)
        .map_err(|e| e.to_string())
}

/// Providers state.
#[derive(Clone, Debug, Default)]
pub struct ProvidersState {
    pub providers: Vec<Provider>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub selected_provider: Option<String>,
    pub selected_model: Option<String>,
}

/// Use providers state hook.
pub fn use_providers_state() -> Signal<ProvidersState> {
    use_signal(ProvidersState::default)
}
