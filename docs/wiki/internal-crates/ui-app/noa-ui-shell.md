# noa-ui-shell Crate

Application shell and layout.

**Location**: `ui/app/crates/noa-ui-shell/`

## Overview

Main application layout components:

- Shell wrapper
- Navigation sidebar
- Header bar
- Status bar

## Components

### Shell

Main layout wrapper.

```rust
#[component]
pub fn Shell(children: Element) -> Element {
    rsx! {
        div { class: "shell",
            Header {}
            div { class: "shell-content",
                Sidebar {}
                main { class: "shell-main",
                    {children}
                }
            }
            StatusBar {}
        }
    }
}
```

### Sidebar

Navigation sidebar.

```rust
#[component]
pub fn Sidebar() -> Element {
    rsx! {
        nav { class: "sidebar",
            NavItem { icon: "home", label: "Dashboard", route: Route::Home }
            NavItem { icon: "agents", label: "Agents", route: Route::Agents }
            NavItem { icon: "tasks", label: "Tasks", route: Route::Tasks }
            NavItem { icon: "settings", label: "Settings", route: Route::Settings }
        }
    }
}
```

### Header

Top header bar.

```rust
#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "header",
            Logo {}
            SearchBar {}
            UserMenu {}
        }
    }
}
```

### StatusBar

Bottom status bar.

```rust
#[component]
pub fn StatusBar() -> Element {
    rsx! {
        footer { class: "status-bar",
            ConnectionStatus {}
            AgentCount {}
            Version {}
        }
    }
}
```

## See Also

- [noa-ui-core](noa-ui-core.md) — Core types
- [noa-ui-styleguide-ui](noa-ui-styleguide-ui.md) — Style viewer
