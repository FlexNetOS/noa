# rust-lovable UI Library

Integration documentation for the rust-lovable component library.

## Overview

rust-lovable is a comprehensive UI building platform that combines conversational AI with cross-platform UI generation. It provides the component foundation for NOA's user interface.

## Location

```
ui/rust-lovable/rust-lovable/
├── src/              # Core library
├── crates/           # Sub-crates
├── examples/         # Usage examples
├── templates/        # UI templates
└── wiki/             # Additional docs
```

## Key Features

- **Conversational AI**: Natural language UI generation
- **Cross-Platform**: Web, Desktop, Mobile targets
- **Component Library**: Pre-built UI components
- **Code Generation**: Automatic code scaffolding
- **Multi-Provider AI**: OpenAI, Anthropic, Groq, Local

## Documentation

### In-Repository Docs

| Document | Description |
|----------|-------------|
| [README](../../../ui/rust-lovable/rust-lovable/README.md) | Project overview |
| [Architecture](../../../ui/rust-lovable/rust-lovable/ARCHITECTURE.md) | System design |
| [API Reference](../../../ui/rust-lovable/rust-lovable/API_REFERENCE.md) | Endpoint documentation |
| [Quick Start](../../../ui/rust-lovable/rust-lovable/wiki/quick-start.md) | Getting started guide |
| [Runbook](../../../ui/rust-lovable/rust-lovable/RUNBOOK.md) | Operational procedures |

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Interface Layer                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Toolbar    │  │  UI Canvas   │  │Chat Interface│          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
├─────────────────────────────────────────────────────────────────┤
│                      Core Business Logic                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │Conversational│  │   UI         │  │  Project     │          │
│  │      AI      │  │ Generator    │  │  Manager     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
├─────────────────────────────────────────────────────────────────┤
│                     Data & Integration Layer                     │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   SQLite     │  │   AI APIs    │  │ Future Int.  │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

## Components

rust-lovable provides these component categories:

| Category | Components |
|----------|------------|
| Layout | Container, Grid, Flex, Stack |
| Forms | Input, Button, Select, Checkbox |
| Display | Text, Heading, Image, Icon |
| Navigation | Link, Menu, Tabs, Breadcrumb |
| Feedback | Alert, Toast, Modal, Progress |

## Usage in NOA

The NOA UI (`ui/app/`) uses rust-lovable components:

```rust
use rust_lovable::components::{Button, Input, Card};

fn MyComponent() -> Element {
    rsx! {
        Card {
            Input { placeholder: "Enter text..." }
            Button { "Submit" }
        }
    }
}
```

## Theming

rust-lovable supports theming via CSS variables:

```css
:root {
    --rl-primary: #3b82f6;
    --rl-secondary: #6366f1;
    --rl-background: #ffffff;
    --rl-text: #1f2937;
}
```

## See Also

- [UI App Crates](ui-app/index.md)
- [Dioxus UI ADR](../../pages/adr/003-dioxus-ui.md)
