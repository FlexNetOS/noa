//! UI Preview component
//!
//! Provides live preview of generated UI components.

use dioxus::prelude::*;

/// UI Preview component for displaying generated interfaces
pub struct UIPreview {
    /// Current preview HTML content
    pub html_content: String,
    /// Current preview CSS
    pub css_content: String,
    /// Preview mode
    pub mode: PreviewMode,
    /// Viewport size
    pub viewport: ViewportSize,
    /// Whether preview is loading
    pub is_loading: bool,
}

/// Preview rendering mode
#[derive(Debug, Clone, PartialEq)]
pub enum PreviewMode {
    /// Live preview
    Live,
    /// Static snapshot
    Static,
    /// Interactive mode
    Interactive,
}

/// Viewport size presets
#[derive(Debug, Clone, PartialEq)]
pub enum ViewportSize {
    /// Mobile (375x667)
    Mobile,
    /// Tablet (768x1024)
    Tablet,
    /// Desktop (1920x1080)
    Desktop,
    /// Custom size
    Custom { width: u32, height: u32 },
}

impl ViewportSize {
    /// Get dimensions as (width, height)
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ViewportSize::Mobile => (375, 667),
            ViewportSize::Tablet => (768, 1024),
            ViewportSize::Desktop => (1920, 1080),
            ViewportSize::Custom { width, height } => (*width, *height),
        }
    }
}

impl UIPreview {
    /// Create a new UI preview
    pub fn new() -> Self {
        Self {
            html_content: String::new(),
            css_content: String::new(),
            mode: PreviewMode::Live,
            viewport: ViewportSize::Desktop,
            is_loading: false,
        }
    }

    /// Set the HTML content to preview
    pub fn set_html(&mut self, html: String) {
        self.html_content = html;
    }

    /// Set the CSS content
    pub fn set_css(&mut self, css: String) {
        self.css_content = css;
    }

    /// Set the viewport size
    pub fn set_viewport(&mut self, viewport: ViewportSize) {
        self.viewport = viewport;
    }

    /// Set preview mode
    pub fn set_mode(&mut self, mode: PreviewMode) {
        self.mode = mode;
    }

    /// Refresh the preview
    pub fn refresh(&mut self) {
        self.is_loading = true;
        self.is_loading = false;
    }
}

impl Default for UIPreview {
    fn default() -> Self {
        Self::new()
    }
}

/// UI Preview Dioxus component
#[component]
pub fn UIPreviewComponent() -> Element {
    let mut selected_viewport = use_signal(|| ViewportSize::Desktop);
    let (width, height) = selected_viewport.read().dimensions();

    rsx! {
        div {
            class: "ui-preview",

            // Toolbar
            div {
                class: "preview-toolbar",

                // Viewport selector
                div {
                    class: "viewport-selector",
                    button {
                        onclick: move |_| selected_viewport.set(ViewportSize::Mobile),
                        "📱 Mobile"
                    }
                    button {
                        onclick: move |_| selected_viewport.set(ViewportSize::Tablet),
                        "📲 Tablet"
                    }
                    button {
                        onclick: move |_| selected_viewport.set(ViewportSize::Desktop),
                        "🖥️ Desktop"
                    }
                }
            }

            // Preview frame
            div {
                class: "preview-frame",
                style: "width: {width}px; height: {height}px;",

                // Render content
                div {
                    class: "preview-content",
                    "Preview content goes here"
                }
            }
        }
    }
}
