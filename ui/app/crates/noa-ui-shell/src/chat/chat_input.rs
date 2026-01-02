//! Chat input component.

use dioxus::prelude::*;

/// Chat input with send button.
#[component]
pub fn ChatInput(
    #[props(default = String::new())] placeholder: String,
    disabled: bool,
    on_send: EventHandler<String>,
) -> Element {
    let mut input_value = use_signal(String::new);
    
    let placeholder_text = if placeholder.is_empty() {
        "Type your message...".to_string()
    } else {
        placeholder
    };
    
    let handle_send = move |_| {
        let value = input_value.read().clone();
        if !value.trim().is_empty() {
            on_send.call(value);
            input_value.set(String::new());
        }
    };
    
    let handle_keydown = move |event: KeyboardEvent| {
        if event.key() == Key::Enter && !event.modifiers().shift() {
            let value = input_value.read().clone();
            if !value.trim().is_empty() {
                on_send.call(value);
                input_value.set(String::new());
            }
        }
    };
    
    rsx! {
        div {
            class: "chat-input-container flex gap-2 p-4 border-t border-base-300",
            
            // Text area
            textarea {
                class: "chat-input flex-1 textarea textarea-bordered resize-none",
                placeholder: "{placeholder_text}",
                value: "{input_value}",
                disabled: disabled,
                rows: "2",
                oninput: move |e| input_value.set(e.value()),
                onkeydown: handle_keydown,
            }
            
            // Send button
            button {
                class: "btn btn-primary self-end",
                disabled: disabled || input_value.read().trim().is_empty(),
                onclick: handle_send,
                
                // Send icon
                svg {
                    class: "w-5 h-5",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                    }
                }
                span { class: "ml-2 hidden sm:inline", "Send" }
            }
        }
    }
}
