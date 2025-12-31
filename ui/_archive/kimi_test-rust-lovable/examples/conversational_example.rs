use rust_lovable::core::conversational_ai::{AIProvider, Conversation, ConversationalAI};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize AI provider
    let ai = ConversationalAI::new(AIProvider::Local {
        endpoint: "http://localhost:8080/ai".to_string(),
    });

    // Create a new conversation
    let mut conversation = Conversation {
        id: uuid::Uuid::new_v4().to_string(),
        messages: Default::default(),
        context: Default::default(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Example user input
    let user_input = "Create a button with the text 'Submit' and make it blue".to_string();

    // Process the message
    let response = ai.process_message(&mut conversation, user_input).await?;

    println!("AI Response: {}", response.content);

    // Parse UI request
    let ui_request = ai.parse_ui_request(&response.content)?;

    println!("UI Change Request: {:?}", ui_request);

    Ok(())
}
