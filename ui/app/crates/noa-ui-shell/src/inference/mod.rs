//! Inference feature module.
//!
//! Provides inference server control and model management.

mod inference_page;
mod server_control;
mod model_selector;
mod completion_panel;

pub use inference_page::InferencePage;
pub use server_control::ServerControl;
pub use model_selector::ModelSelector;
pub use completion_panel::CompletionPanel;
