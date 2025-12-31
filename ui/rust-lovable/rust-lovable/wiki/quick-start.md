# Quick Start Guide

Get up and running with Rust Lovable in just 5 minutes!

## 🚀 Installation

### Single-Click Install

Choose your platform and run the installer:

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/yourusername/rust-lovable/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/yourusername/rust-lovable/main/install.ps1 | iex
```

### Manual Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Clone and Build**:
   ```bash
   git clone https://github.com/yourusername/rust-lovable.git
   cd rust-lovable
   cargo build --release
   ```

3. **Install**:
   ```bash
   sudo cp target/release/rust-lovable /usr/local/bin/
   ```

## 🔧 First Run

### Start the Application

```bash
# Development mode with hot reload
rust-lovable --dev

# Production mode
rust-lovable --release
```

### Configure AI Provider

Create `~/.config/rust-lovable/config.toml`:

```toml
[ai]
provider = "openai"
api_key = "your-api-key-here"
model = "gpt-4"
```

**Get API Keys:**
- [OpenAI](https://platform.openai.com/api-keys)
- [Anthropic](https://console.anthropic.com/)
- [Groq](https://console.groq.com/keys)

## 🎯 Your First Project

### 1. Create a New Project

Open the web interface at `http://localhost:8080` and click "New Project".

Or use the API:
```bash
curl -X POST http://localhost:8080/api/v1/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My First Project",
    "description": "Learning Rust Lovable",
    "platform": "universal"
  }'
```

### 2. Describe Your UI

In the chat interface, type:

```
Create a modern landing page with:
- A hero section with gradient background
- Three feature cards with icons
- A call-to-action button
- Responsive design for mobile and desktop
```

### 3. Watch the Magic Happen

The AI will:
1. Parse your request
2. Generate appropriate components
3. Create responsive styling
4. Show you a live preview

### 4. Refine and Iterate

Try these follow-up commands:

```
Change the hero background to a darker color
Add a navigation bar at the top
Make the feature cards horizontal on desktop
Add hover effects to the buttons
```

## 🎨 Building Your UI

### Basic Commands

**Create Components:**
```
Add a button with text "Sign Up"
Create a form with email and password fields
Add a modal dialog for user settings
```

**Modify Components:**
```
Make the button larger and blue
Change the form labels to be more descriptive
Add validation to the email field
```

**Layout Changes:**
```
Arrange the cards in a grid layout
Add spacing between the sections
Make the header sticky
```

### Platform-Specific Adaptations

**Mobile:**
```
On mobile, make the cards stack vertically
Add touch-friendly button sizes
Hide the sidebar on small screens
```

**Desktop:**
```
On desktop, show a three-column layout
Add keyboard shortcuts
Enable hover effects
```

## 📱 Cross-Platform Development

### Switch Platforms

In the toolbar, select your target platform:
- 🌐 **Web**: Standard web application
- 🖥️ **Desktop**: Native desktop app
- 📱 **Mobile**: Mobile-optimized version
- 🔄 **Universal**: Single codebase, all platforms

### Platform-Specific Features

```
Add a menu bar for desktop
Create a bottom navigation for mobile
Use native file picker for desktop
Add touch gestures for mobile
```

## 🧪 Testing Your UI

### Live Preview

The canvas updates in real-time as you make changes:

1. **Design View**: Visual component tree
2. **Code View**: Generated source code
3. **Split View**: Design + code side by side
4. **Preview Mode**: Interactive preview

### Device Simulation

Test on different devices:
- 📱 iPhone 14 Pro
- 📱 Samsung Galaxy S23
- 🖥️ 1920x1080 Desktop
- 🖥️ 2560x1440 Desktop
- 📱 iPad Pro

## 🚀 Export and Deploy

### Export Options

**As ZIP Archive:**
```bash
curl -X POST http://localhost:8080/api/v1/projects/{id}/export \
  -d '{"format": "zip"}' \
  --output project.zip
```

**To GitHub:**
```bash
curl -X POST http://localhost:8080/api/v1/projects/{id}/export \
  -d '{
    "format": "github",
    "repo": "username/repo",
    "token": "github-token"
  }'
```

**To Vercel:**
```bash
curl -X POST http://localhost:8080/api/v1/projects/{id}/deploy \
  -d '{
    "platform": "vercel",
    "token": "vercel-token"
  }'
```

### Manual Deployment

**Web Deployment:**
```bash
cd project-directory
npm install
npm run build
npm run preview
```

**Desktop Build:**
```bash
cargo tauri build
```

## 🎓 Next Steps

### Learn More

- 📖 [User Interface Guide](user-interface.md)
- 🤖 [Conversational AI](conversational-ai.md)
- 🏗️ [Project Management](project-management.md)
- 🌍 [Cross-Platform Development](cross-platform.md)

### Advanced Features

- 🔌 [Custom Components](custom-components.md)
- ⚙️ [Configuration Options](configuration.md)
- 🔧 [Plugin Development](plugin-development.md)
- 📊 [Performance Optimization](performance.md)

### Get Help

- 💬 [Join our Discord](https://discord.gg/rust-lovable)
- 🐛 [Report Issues](https://github.com/yourusername/rust-lovable/issues)
- 📧 [Email Support](mailto:support@rust-lovable.com)

## 🎯 Tips and Tricks

### 1. Be Specific in Your Requests

❌ "Make it better"
✅ "Increase the font size to 18px and change the color to blue"

### 2. Use Iterative Refinement

Start simple and add complexity:
1. "Create a button"
2. "Make it blue"
3. "Add hover effects"
4. "Make it responsive"

### 3. Leverage Context

The AI remembers your project context:
- Component names and types
- Styling preferences
- Platform targets
- Previous interactions

### 4. Platform-Specific Commands

Use platform-specific language:
- "On mobile, make the buttons larger"
- "For desktop, add a keyboard shortcut"
- "On tablets, show a sidebar"

### 5. Component Reuse

Reference existing components:
- "Make this button like the other one"
- "Use the same styling as the header"
- "Copy the layout from the home page"

## 🐛 Common Issues

### Issue: "AI not responding"

**Solution:**
1. Check your API key in config.toml
2. Verify internet connection
3. Try a different AI provider
4. Check API rate limits

### Issue: "Sandbox execution failed"

**Solution:**
1. Check code syntax
2. Verify package dependencies
3. Increase timeout limits
4. Check sandbox logs

### Issue: "UI not updating"

**Solution:**
1. Refresh the browser
2. Check WebSocket connection
3. Verify component properties
4. Check for JavaScript errors

## 📊 Performance Tips

### 1. Cache Frequently Used Components

```toml
# config.toml
[performance]
cache_components = true
cache_size = "1GB"
```

### 2. Optimize AI Requests

```toml
[ai]
cache_responses = true
batch_requests = true
```

### 3. Use Production Builds

```bash
# For better performance
rust-lovable --release
```

## 🎉 Success Stories

See what others have built with Rust Lovable:

- **E-commerce Platform**: Complete online store with cart and checkout
- **Dashboard**: Real-time analytics dashboard with charts
- **Portfolio Site**: Professional portfolio with animations
- **Blog**: Content management system with markdown support
- **Social App**: Twitter-like social media application

---

Congratulations! You're now ready to build amazing UIs with Rust Lovable. Start creating! 🚀