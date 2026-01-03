//! Code editor component
//!
//! Provides an integrated code editor with syntax highlighting and AI assistance.

use dioxus::prelude::*;

/// Code editor component
pub struct CodeEditor {
    /// Current file content
    pub content: String,
    /// Current file path
    pub file_path: Option<String>,
    /// Current language
    pub language: String,
    /// Cursor position (line, column)
    pub cursor_position: (usize, usize),
    /// Selection range (start, end)
    pub selection: Option<(usize, usize)>,
    /// Unsaved changes
    pub is_dirty: bool,
}

impl CodeEditor {
    /// Create a new code editor
    pub fn new() -> Self {
        Self {
            content: String::new(),
            file_path: None,
            language: "plaintext".to_string(),
            cursor_position: (0, 0),
            selection: None,
            is_dirty: false,
        }
    }

    /// Open a file in the editor
    pub fn open_file(&mut self, path: &str, content: String) {
        self.file_path = Some(path.to_string());
        self.content = content;
        self.language = detect_language(path);
        self.cursor_position = (0, 0);
        self.selection = None;
        self.is_dirty = false;
    }

    /// Set content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.is_dirty = true;
    }

    /// Get content
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Insert text at cursor position
    pub fn insert_at_cursor(&mut self, text: &str) {
        self.content.push_str(text);
        self.is_dirty = true;
    }
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect language from file extension
fn detect_language(path: &str) -> String {
    let extension = path.split('.').last().unwrap_or("");
    match extension {
        "rs" => "rust",
        "js" | "jsx" => "javascript",
        "ts" | "tsx" => "typescript",
        "py" => "python",
        "html" => "html",
        "css" => "css",
        "json" => "json",
        "md" => "markdown",
        "toml" => "toml",
        "yaml" | "yml" => "yaml",
        _ => "plaintext",
    }
    .to_string()
}

/// Code editor Dioxus component
#[component]
pub fn CodeEditorComponent(initial_content: Option<String>, language: Option<String>) -> Element {
    let mut content = use_signal(|| initial_content.unwrap_or_default());
    let lang = language.unwrap_or_else(|| "plaintext".to_string());

    rsx! {
        div {
            class: "code-editor",

            // Editor header
            div {
                class: "editor-header",
                span { class: "language-badge", "{lang}" }
            }

            // Editor content
            textarea {
                class: "editor-content",
                value: "{content}",
                oninput: move |e| {
                    content.set(e.value().clone());
                },
                spellcheck: false,
            }
        }
    }
}
