# Rust Lovable

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.6-blue.svg)](https://dioxuslabs.com/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/yourusername/rust-lovable/build.yml?branch=main)](https://github.com/yourusername/rust-lovable/actions)

> A comprehensive Rust-based mirror of Lovable.dev with conversational UI capabilities, cross-platform dynamic UX, and all Lovable features.

## 🚀 Features

- **Conversational UI**: Request UI changes through natural language
- **Cross-Platform**: Dynamic UX that adapts across Web, Desktop, and Mobile
- **AI-Powered**: Multi-provider AI integration (OpenAI, Anthropic, Groq)
- **Real-time Streaming**: Server-sent events for live code generation
- **Sandbox Environment**: Secure code execution with resource limits
- **Package Management**: Automatic dependency detection and installation
- **Export Capabilities**: ZIP, GitHub, and Vercel deployment
- **Real-time Collaboration**: Multi-user editing capabilities
- **Brand Style Extraction**: Extract design systems from websites
- **Vite Integration**: Live error monitoring and build optimization

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [API Reference](#api-reference)
- [Development](#development)
- [Testing](#testing)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [License](#license)

## 🛠 Installation

### Prerequisites

- **Rust** 1.75.0 or higher
- **System dependencies** (platform-specific):
  - **Linux**: `build-essential`, `pkg-config`, `libssl-dev`
  - **macOS**: Xcode Command Line Tools, `brew`
  - **Windows**: Visual Studio Build Tools

### Single-Click Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-lovable.git
cd rust-lovable

# Run the enhanced installation script
./install-v2.sh
```

The installation script will:
- Detect your hardware and platform
- Install system dependencies
- Install/verify Rust toolchain
- Build Rust Lovable optimized for your hardware
- Create configuration files
- Set up monitoring and health checks
- Add to PATH

### Manual Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/yourusername/rust-lovable.git
cd rust-lovable
cargo build --release

# Install binary
cp target/release/rust-lovable $HOME/.local/bin/
```

### Docker Installation

```bash
# Build Docker image
docker build -t rust-lovable .

# Run container
docker run -p 8080:8080 rust-lovable
```

## 🚀 Quick Start

### Starting the Application

```bash
# Development mode
rust-lovable --dev

# Production mode
rust-lovable --release

# With custom config
rust-lovable --config /path/to/config.toml
```

### Creating Your First Project

```bash
# Create a new project
rust-lovable project create --name "My Awesome App" --platform web

# Start interactive mode
rust-lovable interactive

# In interactive mode, you can say:
# "Create a modern landing page with a hero section"
```

### Using the API

```bash
# Create project via API
curl -X POST http://localhost:8080/api/v1/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My App",
    "description": "A modern web app",
    "platform": "web"
  }'
```

## 🏗 Architecture

Rust Lovable is built with a modular architecture designed for scalability and extensibility:

```
rust-lovable/
├── src/
│   ├── core/                 # Core business logic
│   │   ├── conversational_ai.rs
│   │   ├── ui_generator.rs
│   │   ├── project_manager.rs
│   │   ├── cross_platform.rs
│   │   └── code_generator.rs
│   ├── components/           # UI components
│   │   ├── chat.rs
│   │   ├── canvas.rs
│   │   ├── sidebar.rs
│   │   └── toolbar.rs
│   ├── utils/               # Utility functions
│   └── app.rs              # Main application
├── src-tauri/               # Tauri desktop app
│   └── src/
│       ├── api/            # API endpoints
│       ├── sandbox/        # Sandbox management
│       └── main.rs
├── tests/                  # Comprehensive tests
├── wiki/                   # Documentation
└── scripts/               # Build and deployment scripts
```

### Core Components

- **Conversational AI**: Natural language processing and UI generation
- **UI Generator**: Dynamic component creation and modification
- **Project Manager**: Project lifecycle and state management
- **Cross-Platform Adapter**: Platform-specific optimizations
- **Code Generator**: Multi-language code generation
- **Sandbox Manager**: Secure code execution environment

## 🔌 API Reference

Complete API documentation is available in [API_REFERENCE.md](API_REFERENCE.md).

### Key Endpoints

- `POST /api/v1/projects` - Create project
- `POST /api/v1/ai/process` - Process natural language
- `GET /api/v1/stream/ai/generate/{id}` - Stream AI code generation
- `POST /api/v1/sandboxes` - Create sandbox
- `POST /api/v1/export/github` - Export to GitHub

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run specific test suite
cargo test --test comprehensive_tests

# Run integration tests
cargo test --test integration_tests
```

### Test Coverage

- **Unit Tests**: Core functionality testing
- **Integration Tests**: API endpoint testing
- **End-to-End Tests**: Full workflow testing
- **Performance Tests**: Benchmarking and optimization
- **Security Tests**: Vulnerability assessment

### Verification Script

```bash
# Run installation verification
./verify-installation.sh
```

## 🛠 Development

### Development Setup

```bash
# Clone repository
git clone https://github.com/yourusername/rust-lovable.git
cd rust-lovable

# Install dependencies
cargo install cargo-watch cargo-tarpaulin

# Start development server
cargo watch -x run
```

### Code Style

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy
```

### Architecture Guidelines

- **Modular Design**: Keep components focused and reusable
- **Error Handling**: Use `Result<T, E>` for error propagation
- **Async/Await**: Leverage Tokio for concurrent operations
- **Type Safety**: Use Rust's type system for compile-time guarantees
- **Performance**: Profile and optimize critical paths

## 🚀 Deployment

### Production Deployment

```bash
# Build for production
cargo build --release

# Run with production config
./target/release/rust-lovable --release --config prod.toml
```

### Docker Deployment

```bash
# Build production image
docker build -f Dockerfile.prod -t rust-lovable:prod .

# Run production container
docker run -d -p 80:8080 --name rust-lovable rust-lovable:prod
```

### Kubernetes Deployment

```bash
# Apply Kubernetes manifests
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -l app=rust-lovable
```

## 📚 Documentation

- **[Quick Start Guide](wiki/quick-start.md)** - Get up and running quickly
- **[Architecture Overview](ARCHITECTURE.md)** - System design and components
- **[API Reference](API_REFERENCE.md)** - Complete API documentation
- **[Runbook](RUNBOOK-V2.md)** - Operations and troubleshooting
- **[Gap Analysis](GAP_ANALYSIS.md)** - Feature comparison and roadmap

## 🤝 Contributing

We welcome contributions from the community! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## 🗓 Roadmap

### Q1 2024
- [ ] Advanced AI integration with multiple providers
- [ ] Real-time collaboration features
- [ ] Mobile platform optimizations

### Q2 2024
- [ ] Plugin ecosystem
- [ ] Advanced export options
- [ ] Enterprise features

### Q3 2024
- [ ] Performance optimizations
- [ ] Advanced debugging tools
- [ ] Community marketplace

## 🆘 Support

### Getting Help

- **Documentation**: https://docs.rust-lovable.com
- **GitHub Issues**: [Create an issue](https://github.com/yourusername/rust-lovable/issues)
- **Discord Community**: [Join our Discord](https://discord.gg/rust-lovable)
- **Email**: support@rust-lovable.com

### Troubleshooting

Common issues and solutions:

1. **Build failures**: Ensure all system dependencies are installed
2. **Runtime errors**: Check logs in `$HOME/.local/share/rust-lovable/logs/`
3. **Permission issues**: Verify file permissions and PATH configuration
4. **Network issues**: Check firewall and proxy settings

## 📊 Metrics and Monitoring

### Built-in Metrics

- Request rate and latency
- Error rates and types
- Resource utilization
- AI generation statistics

### Health Checks

```bash
# Check application health
curl http://localhost:8080/api/v1/health

# Get detailed metrics
curl http://localhost:8080/api/v1/metrics
```

### Monitoring Integration

- **Prometheus**: Metrics collection
- **Grafana**: Visualization and alerting
- **Jaeger**: Distributed tracing
- **ELK Stack**: Log aggregation

## 🔒 Security

### Security Features

- Input validation and sanitization
- Rate limiting and DDoS protection
- Secure sandbox execution
- API key authentication
- Audit logging

### Security Policy

Please see our [Security Policy](SECURITY.md) for details on reporting security vulnerabilities.

## 🏛 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Dioxus Team**: For the amazing cross-platform UI framework
- **Rust Community**: For the excellent tooling and ecosystem
- **Open Source Contributors**: For making this project possible

## 📈 Performance

### Benchmarks

- **API Response Time**: < 100ms average
- **AI Generation**: < 5 seconds for complex UIs
- **Build Time**: < 30 seconds for large projects
- **Memory Usage**: < 500MB idle, < 2GB under load

### Scalability

- Horizontal scaling with load balancers
- Database connection pooling
- Caching strategies
- Resource limits and monitoring

---

**Built with ❤️ by the Rust Lovable team**

*Empowering developers to build amazing UIs with the power of Rust and AI.*