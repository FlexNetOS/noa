//! NOA Unified Component Library
//!
//! This crate provides a unified interface to both:
//! - `dioxus-primitives`: Unstyled, ARIA-compliant behavior components
//! - `daisy_rsx`: Pre-styled DaisyUI components for rapid prototyping
//!
//! # Usage
//!
//! ```rust,ignore
//! use noa_ui_components::prelude::*;
//!
//! // Use primitives for custom-styled ARIA-compliant components
//! use noa_ui_components::primitives::{Accordion, Dialog, Tabs};
//!
//! // Use daisy for pre-styled components
//! use noa_ui_components::daisy::{Button, Card, ChatBubble};
//! ```
//!
//! # Design Philosophy
//!
//! - **Primitives**: Use for core behaviors when you need full styling control
//! - **Daisy**: Use for rapid prototyping and consistent DaisyUI theming
//! - **Wrappers**: NOA-specific components that combine both libraries

#[cfg(feature = "primitives")]
pub mod primitives {
    //! Re-exports from dioxus-primitives for ARIA-compliant unstyled components.
    //!
    //! These components provide accessibility behaviors without styling,
    //! allowing full customization while maintaining ARIA compliance.

    pub use dioxus_primitives::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
    pub use dioxus_primitives::alert_dialog::{AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogTitle, AlertDialogTrigger};
    pub use dioxus_primitives::avatar::{Avatar, AvatarFallback, AvatarImage};
    pub use dioxus_primitives::calendar::Calendar;
    pub use dioxus_primitives::checkbox::Checkbox;
    pub use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
    pub use dioxus_primitives::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};
    pub use dioxus_primitives::dialog::{Dialog, DialogClose, DialogContent, DialogDescription, DialogTitle, DialogTrigger};
    pub use dioxus_primitives::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
    pub use dioxus_primitives::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};
    pub use dioxus_primitives::label::Label;
    pub use dioxus_primitives::menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger};
    pub use dioxus_primitives::navbar::{Navbar, NavbarBrand, NavbarContent, NavbarItem};
    pub use dioxus_primitives::popover::{Popover, PopoverContent, PopoverTrigger};
    pub use dioxus_primitives::progress::Progress;
    pub use dioxus_primitives::radio_group::{RadioGroup, RadioGroupItem};
    pub use dioxus_primitives::scroll_area::ScrollArea;
    pub use dioxus_primitives::separator::Separator;
    pub use dioxus_primitives::slider::Slider;
    pub use dioxus_primitives::switch::Switch;
    pub use dioxus_primitives::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
    pub use dioxus_primitives::toast::{Toast, ToastAction, ToastClose, ToastDescription, ToastProvider, ToastTitle, ToastViewport};
    pub use dioxus_primitives::toggle::Toggle;
    pub use dioxus_primitives::toggle_group::{ToggleGroup, ToggleGroupItem};
    pub use dioxus_primitives::toolbar::{Toolbar, ToolbarButton, ToolbarLink, ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem};
    pub use dioxus_primitives::tooltip::{Tooltip, TooltipContent, TooltipProvider, TooltipTrigger};
}

#[cfg(feature = "daisy")]
pub mod daisy {
    //! Re-exports from daisy_rsx for pre-styled DaisyUI components.
    //!
    //! These components come with DaisyUI styling out of the box,
    //! ideal for rapid prototyping and consistent theming.

    // Layout
    pub use daisy_rsx::{Card, CardBody, CardActions};
    pub use daisy_rsx::{Drawer, DrawerSide, DrawerContent};
    pub use daisy_rsx::{Modal, ModalAction, ModalBody};

    // Navigation
    pub use daisy_rsx::{Navbar as DaisyNavbar};
    pub use daisy_rsx::{Menu, MenuItem, MenuTitle};
    pub use daisy_rsx::{Breadcrumb, BreadcrumbItem};
    pub use daisy_rsx::{Tabs as DaisyTabs, Tab};

    // Data Display
    pub use daisy_rsx::{Avatar as DaisyAvatar};
    pub use daisy_rsx::{Badge};
    pub use daisy_rsx::{Table, TableHead, TableBody, TableRow, TableCell};
    pub use daisy_rsx::{Collapse, CollapseContent};

    // Data Input
    pub use daisy_rsx::{Button, ButtonGroup};
    pub use daisy_rsx::{Input, TextArea};
    pub use daisy_rsx::{Select, SelectOption};
    pub use daisy_rsx::{Checkbox as DaisyCheckbox};
    pub use daisy_rsx::{Radio};
    pub use daisy_rsx::{Toggle as DaisyToggle};
    pub use daisy_rsx::{Range};
    pub use daisy_rsx::{Rating};
    pub use daisy_rsx::{FileInput};

    // Feedback
    pub use daisy_rsx::{Alert};
    pub use daisy_rsx::{Loading, LoadingType};
    pub use daisy_rsx::{Progress as DaisyProgress};
    pub use daisy_rsx::{Tooltip as DaisyTooltip};
    pub use daisy_rsx::{Toast as DaisyToast};

    // Chat
    pub use daisy_rsx::{ChatBubble, ChatHeader, ChatFooter, ChatImage};

    // Mockup
    pub use daisy_rsx::{CodeBlock};
}

/// NOA-specific component wrappers that combine primitives and daisy styling.
pub mod noa {
    use dioxus::prelude::*;

    /// NOA color palette for consistent theming.
    #[derive(Clone, Copy, PartialEq, Default)]
    pub enum NoaColor {
        #[default]
        Primary,
        Secondary,
        Accent,
        Neutral,
        Success,
        Warning,
        Error,
        Info,
    }

    impl NoaColor {
        /// Returns the DaisyUI color class.
        pub fn to_class(&self) -> &'static str {
            match self {
                NoaColor::Primary => "btn-primary",
                NoaColor::Secondary => "btn-secondary",
                NoaColor::Accent => "btn-accent",
                NoaColor::Neutral => "btn-neutral",
                NoaColor::Success => "btn-success",
                NoaColor::Warning => "btn-warning",
                NoaColor::Error => "btn-error",
                NoaColor::Info => "btn-info",
            }
        }
    }

    /// NOA-styled button with consistent theming.
    #[component]
    pub fn NoaButton(
        /// Button label text.
        label: String,
        /// Color variant.
        #[props(default)]
        color: NoaColor,
        /// Whether the button is disabled.
        #[props(default = false)]
        disabled: bool,
        /// Whether to show loading state.
        #[props(default = false)]
        loading: bool,
        /// Click handler.
        onclick: Option<EventHandler<MouseEvent>>,
    ) -> Element {
        let class = format!("btn {} {}", 
            color.to_class(),
            if loading { "loading" } else { "" }
        );

        rsx! {
            button {
                class: "{class}",
                disabled: disabled || loading,
                onclick: move |evt| {
                    if let Some(handler) = &onclick {
                        handler.call(evt);
                    }
                },
                "{label}"
            }
        }
    }

    /// NOA-styled chat message bubble.
    #[component]
    pub fn NoaChatMessage(
        /// Message content.
        content: String,
        /// Whether this is from the user (right-aligned) or assistant (left-aligned).
        #[props(default = false)]
        is_user: bool,
        /// Optional timestamp.
        timestamp: Option<String>,
    ) -> Element {
        let align_class = if is_user { "chat-end" } else { "chat-start" };
        let bubble_class = if is_user { "chat-bubble-primary" } else { "chat-bubble-secondary" };

        rsx! {
            div { class: "chat {align_class}",
                div { class: "chat-bubble {bubble_class}",
                    "{content}"
                }
                if let Some(ts) = timestamp {
                    div { class: "chat-footer opacity-50",
                        "{ts}"
                    }
                }
            }
        }
    }

    /// NOA-styled scrollable chat container using primitives::ScrollArea.
    #[component]
    pub fn NoaChatContainer(
        /// Chat messages as children.
        children: Element,
    ) -> Element {
        rsx! {
            div { class: "flex flex-col h-full",
                div { class: "flex-1 overflow-y-auto p-4 space-y-4",
                    {children}
                }
            }
        }
    }

    /// NOA-styled model/provider selector tabs.
    #[component]
    pub fn NoaProviderTabs(
        /// Available providers.
        providers: Vec<String>,
        /// Currently selected provider index.
        selected: usize,
        /// Selection change handler.
        on_select: EventHandler<usize>,
    ) -> Element {
        rsx! {
            div { class: "tabs tabs-boxed",
                for (idx, provider) in providers.iter().enumerate() {
                    a {
                        class: if idx == selected { "tab tab-active" } else { "tab" },
                        onclick: move |_| on_select.call(idx),
                        "{provider}"
                    }
                }
            }
        }
    }
}

/// Prelude for convenient imports.
pub mod prelude {
    #[cfg(feature = "primitives")]
    pub use super::primitives;

    #[cfg(feature = "daisy")]
    pub use super::daisy;

    pub use super::noa::*;
}
