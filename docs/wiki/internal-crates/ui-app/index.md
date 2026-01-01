# ui-app Crates

NOA's user interface layer, built with Dioxus and Tauri.

**Location**: `ui/app/`  
**Framework**: [Dioxus](https://dioxuslabs.com/) 0.7.2  
**Desktop**: [Tauri](https://tauri.app/) 2.x  
**Edition**: Rust 2024  

## Workspace Structure

```
ui/app/
├── crates/           # Shared library crates
│   ├── noa-ui-core/
│   ├── noa-ui-paths/
│   ├── noa-ui-protocol/
│   ├── noa-ui-shell/
│   ├── noa-ui-styleguide-api/
│   └── noa-ui-styleguide-ui/
├── bins/             # Binary targets
│   ├── noa-ui-desktop/
│   ├── noa-ui-web/
│   └── noa-ui-hived/
├── pages/            # Route components
├── shell/            # Layout & navigation
├── widget/           # Reusable widgets
└── src/              # Main entry points
```

## Library Crates

### noa-ui-core

Core utilities and types shared across all UI crates.

| Module | Purpose |
|--------|---------|
| `state` | Global application state |
| `hooks` | Custom Dioxus hooks |
| `types` | Shared type definitions |

### noa-ui-paths

Route definitions and path utilities.

| Export | Purpose |
|--------|---------|
| `Route` | Enum of all application routes |
| `paths::*` | URL path constants |

### noa-ui-protocol

Communication protocol with sys-core backend.

| Module | Purpose |
|--------|---------|
| `commands` | Tauri command definitions |
| `events` | Backend event types |
| `messages` | IPC message formats |

### noa-ui-shell

Application shell layout and navigation.

| Component | Purpose |
|-----------|---------|
| `Shell` | Main layout wrapper |
| `Sidebar` | Navigation sidebar |
| `Header` | Top header bar |
| `StatusBar` | Bottom status bar |

### noa-ui-styleguide-api

Style system API and tokens.

| Export | Purpose |
|--------|---------|
| `colors` | Color palette tokens |
| `spacing` | Spacing scale |
| `typography` | Font definitions |
| `shadows` | Shadow presets |

### noa-ui-styleguide-ui

Styleguide viewer UI component.

| Component | Purpose |
|-----------|---------|
| `Styleguide` | Interactive style browser |

## Binary Targets

### noa-ui-desktop

Tauri desktop application.

```bash
# Development
cargo tauri dev

# Production build
cargo tauri build
```

**Platforms**: Windows, macOS, Linux

### noa-ui-web

WASM web application.

```bash
# Development
dx serve --platform web

# Production build
dx build --platform web --release
```

### noa-ui-hived

Headless UI for P2P hive nodes.

```bash
# Run hive daemon
cargo run --bin noa-ui-hived
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        noa-ui-desktop                            │
│                              │                                   │
│         ┌────────────────────┼────────────────────┐              │
│         ▼                    ▼                    ▼              │
│   noa-ui-shell         noa-ui-core         noa-ui-protocol      │
│         │                    │                    │              │
│         └──────────┬─────────┴────────────────────┘              │
│                    ▼                                             │
│              noa-ui-paths                                        │
│                    │                                             │
│                    ▼                                             │
│         noa-ui-styleguide-api                                    │
│                    │                                             │
│                    ▼                                             │
│         noa-ui-styleguide-ui                                     │
└─────────────────────────────────────────────────────────────────┘
```

## Component Library

From `rust-lovable` integration:

| Category | Components |
|----------|------------|
| Layout | `Container`, `Grid`, `Stack`, `Box` |
| Navigation | `Navbar`, `Sidebar`, `Breadcrumb`, `Tabs` |
| Forms | `Input`, `Select`, `Checkbox`, `Radio`, `Button` |
| Feedback | `Toast`, `Modal`, `Alert`, `Progress` |
| Data | `Table`, `Card`, `List`, `Badge` |
| Charts | `LineChart`, `BarChart`, `PieChart` |

## Theming

```rust
use noa_ui_styleguide_api::{Theme, ThemeProvider};

fn App(cx: Scope) -> Element {
    render! {
        ThemeProvider { theme: Theme::Dark,
            Shell {
                // App content
            }
        }
    }
}
```

## Hive Protocol

See [HIVE_PROTOCOL.md](../../../../ui/app/HIVE_PROTOCOL.md) for P2P UI synchronization.

---

*Built with Dioxus + Tauri + rust-lovable components*
