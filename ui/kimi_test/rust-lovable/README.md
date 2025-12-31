# Rust Lovable

A Rust-based mirror of Lovable.dev with conversational UI capabilities and dynamic cross-platform UX using Dioxus.

## Features

### Core Functionality
- **Conversational UI Builder**: Describe UI changes in natural language and watch them appear in real-time
- **Cross-Platform Support**: Build once, deploy everywhere (Web, Desktop, Mobile)
- **Dynamic UI Generation**: AI-powered code generation for Dioxus components
- **Live Preview**: See your changes instantly across different platforms
- **Project Management**: Organize your UI projects with pages, components, and assets

### AI Integration
- **Multi-Provider Support**: OpenAI, Anthropic, Groq, or local AI endpoints
- **Natural Language Processing**: Convert conversational descriptions into structured UI changes
- **Context Awareness**: AI remembers project context and user preferences
- **Code Generation**: Automatic generation of clean, maintainable Dioxus code

### Platform Targets
- **Web**: Deploy as a web application with full Dioxus web support
- **Desktop**: Native desktop applications with Tauri integration
- **Mobile**: Mobile apps with responsive design adaptations
- **Universal**: Single codebase that adapts to all platforms

### Future Integrations
The architecture is designed to support future integrations with:
- **ruv-FANN**: Fast Artificial Neural Networks for advanced AI features
- **ruvector**: Distributed vector database for semantic search
- **QuDAG**: Quantum-resistant DAG for distributed coordination
- **ruv-swarm**: Ephemeral swarm intelligence
- **jj**: Advanced version control
- **libp2p**: Peer-to-peer networking

## Quick Start

### Prerequisites
- Rust toolchain (recommended: current stable; **Rust 1.77+ required** if `Cargo.lock` is format v4)
- `wasm32-unknown-unknown` target for web (`rustup target add wasm32-unknown-unknown`)
- Dioxus CLI (`dx`) (installed by `make install`)
- Node.js 18+ (recommended for web asset pipelines; required if your setup uses npm-based tools)

Platform-specific:
- **Windows desktop**: Microsoft Edge **WebView2 Runtime** installed
- **Docker**: Docker Desktop (WSL2 backend recommended)
- **Mobile**:
    - Android builds require Android Studio + SDK/NDK
    - iOS builds require Xcode (macOS)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-lovable.git
cd rust-lovable
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

### Development

For development with hot reload (recommended):

- Web (client-only): `make dev-web`
- Desktop (client-only): `make dev-desktop`

Fullstack (server + client) with hot reload:

- Web fullstack: `make dev-fullstack-web`
- Desktop fullstack: `make dev-fullstack-desktop`

Server-only run targets:

- Run: `make run-server`
- Dev autorestart: `make dev-server`

Server bind address (useful for Docker/deploy): set `RUST_LOVABLE_ADDRESS` (e.g. `0.0.0.0:8080`).

## Architecture

### Core Modules

- **Conversational AI** (`src/core/conversational_ai.rs`): Natural language processing and AI provider integration
- **UI Generator** (`src/core/ui_generator.rs`): Dynamic UI component generation from structured requests
- **Project Manager** (`src/core/project_manager.rs`): Project lifecycle management and persistence
- **Code Generator** (`src/core/code_generator.rs`): Platform-specific code generation
- **Cross-Platform Adapter** (`src/core/cross_platform.rs`): Platform-specific UI adaptations

### UI Components

- **Chat Interface** (`src/components/chat.rs`): Conversational UI for describing changes
- **UI Canvas** (`src/components/canvas.rs`): Visual editor with design/code/preview modes
- **Sidebar** (`src/components/sidebar.rs`): Component library and project explorer
- **Toolbar** (`src/components/toolbar.rs`): Project controls and build tools

### How It Works

1. **User Input**: User describes UI changes through natural language in the chat interface
2. **AI Processing**: Conversational AI parses the request into structured UI change requests
3. **Code Generation**: UI generator creates platform-specific Dioxus components
4. **Live Preview**: Changes are rendered instantly in the canvas preview
5. **Cross-Platform**: Components automatically adapt to selected platform targets

## Usage Examples

### Creating a Button
```
User: "Add a blue button that says 'Submit'"
AI: I'll create a blue button with the text 'Submit' for you.
Result: A blue button component is generated and added to the UI
```

### Modifying Components
```
User: "Make that button larger and change the text to 'Save'"
AI: I'll update the button to make it larger and change the text to 'Save'.
Result: The existing button is modified with new properties
```

### Platform-Specific Changes
```
User: "On mobile, make the button full width"
AI: I'll add a mobile-specific adaptation to make the button full width on mobile devices.
Result: Responsive styling is added for mobile platforms
```

## Configuration

### AI Provider Setup

Configure your AI provider in `config.toml`:

```toml
[ai]
provider = "openai" # or "anthropic", "groq", "local"
api_key = "your-api-key"
model = "gpt-4" # or "claude-3", "llama3", etc.
```

### Platform Settings

```toml
[platform]
targets = ["web", "desktop", "mobile"] # or ["universal"]
default_framework = "dioxus"
theme = "default"
```

## API Reference

### Conversational AI

```rust
use rust_lovable::core::conversational_ai::{ConversationalAI, AIProvider};

let ai = ConversationalAI::new(AIProvider::OpenAI {
    api_key: "your-key".to_string(),
    model: "gpt-4".to_string(),
});

let response = ai.process_message(&mut conversation, user_input).await?;
let ui_request = ai.parse_ui_request(&response.content)?;
```

### UI Generation

```rust
use rust_lovable::core::ui_generator::UIGenerator;

let mut generator = UIGenerator::new();
let component = generator.generate_component(ui_request)?;
let code = generator.generate_code(&component, PlatformTarget::Web)?;
```

## Building for Production

### Web Build
```bash
cargo build --release --features web
target/release/rust-lovable
```

### Desktop Build
```bash
cargo build --release --features desktop
```

### Mobile Build
```bash
cargo build --release --features mobile
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Code Style

- Follow Rust's official style guidelines
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add tests for new functionality

## Roadmap

### Phase 1: Core Features ✅
- [x] Basic conversational UI
- [x] Dynamic component generation
- [x] Cross-platform support
- [x] Live preview
- [x] Project management

### Phase 2: Advanced AI
- [ ] Integration with ruv-FANN for neural networks
- [ ] Semantic search with ruvector
- [ ] Advanced NLP capabilities
- [ ] AI-assisted design suggestions

### Phase 3: Distributed Features
- [ ] QuDAG integration for distributed coordination
- [ ] Peer-to-peer collaboration with libp2p
- [ ] Version control with jj integration
- [ ] Real-time collaborative editing

### Phase 4: Ecosystem
- [ ] Plugin system
- [ ] Marketplace for components
- [ ] Advanced theming
- [ ] Enterprise features

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Lovable.dev](https://lovable.dev/) and [Open Lovable](https://github.com/firecrawl/open-lovable)
- Built with [Dioxus](https://dioxuslabs.com/) for cross-platform UI
- AI capabilities powered by various LLM providers
- Community-driven development approach

## Support

- 📖 [Documentation](https://docs.rust-lovable.com)
- 💬 [Discord Community](https://discord.gg/rust-lovable)
- 🐛 [Issue Tracker](https://github.com/yourusername/rust-lovable/issues)
- 📧 [Email Support](mailto:support@rust-lovable.com)