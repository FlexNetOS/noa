# Quick Start Guide

**ML DevOps Platform v0.3.0**  
**Get running in 5 minutes**

---

## 🚀 30-Second Quick Start

```bash
# 1. Install dependencies
cd ml_devops_platform/nextjs_space
yarn install

# 2. Set up environment
cp .env.example .env
# Add your API key: ABACUSAI_API_KEY=your_key_here

# 3. Initialize database
yarn prisma generate
yarn prisma db push

# 4. Run development server
yarn dev
```

🎉 **Open http://localhost:3000** - You're live!

---

## 📝 Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [First Steps](#first-steps)
- [Key Features](#key-features)
- [Common Tasks](#common-tasks)
- [Troubleshooting](#troubleshooting)
- [Next Steps](#next-steps)

---

## Prerequisites

### Required
- **Node.js** 18+ ([Download](https://nodejs.org/))
- **Yarn** package manager ([Install](https://yarnpkg.com/getting-started/install))
- **PostgreSQL** database (hosted or local)

### Optional
- **Rust** 1.70+ for local inference ([Install](https://rustup.rs/))
- **Docker** for containerized deployment
- **Git** for version control

### Check Your Setup

```bash
node --version  # Should be v18.x or higher
yarn --version  # Should be 1.22.x or higher
rustc --version # (Optional) Should be 1.70.x or higher
```

---

## Installation

### Step 1: Clone & Setup

```bash
# If you haven't cloned yet
git clone <your-repo-url>
cd ml_devops_platform

# Navigate to Next.js app
cd nextjs_space

# Install dependencies
yarn install
```

### Step 2: Environment Configuration

```bash
# Copy example environment file
cp .env.example .env

# Edit .env with your credentials
nano .env  # or use your favorite editor
```

**Required Environment Variables**:

```bash
# Database (provided by Abacus AI)
DATABASE_URL="postgresql://postgres:..."

# Authentication (auto-generated on first run)
NEXTAUTH_SECRET="your-secret-key"
NEXTAUTH_URL="http://localhost:3000"

# AI Provider (required)
ABACUSAI_API_KEY="your-api-key-here"

# Analytics (optional)
NEXT_PUBLIC_UMAMI_WEBSITE_ID="your-website-id"
NEXT_PUBLIC_UMAMI_URL="https://analytics.example.com"
```

### Step 3: Database Setup

```bash
# Generate Prisma client
yarn prisma generate

# Push schema to database
yarn prisma db push

# (Optional) Seed database with test data
yarn prisma db seed
```

### Step 4: Start Development Server

```bash
# Start Next.js development server
yarn dev

# Server will start on http://localhost:3000
```

---

## First Steps

### 1. Create Your Account

1. Navigate to **http://localhost:3000**
2. You'll be redirected to **Login** page
3. Click **"Sign Up"**
4. Fill in:
   - Email: `your@email.com`
   - Password: `SecurePassword123`
   - Name: `Your Name`
5. Click **"Create Account"**

### 2. Explore the Dashboard

**Main Dashboard** (`/`):
- ✅ **Unified Chat** - Conversational AI interface
- ✅ **MOE System** - Intelligent provider routing
- ✅ **Quick Links** - DeepCode, SONA, Documentation

### 3. Try Your First Chat

1. In the **Unified Chat** interface:
   ```
   User: Write a React component for a todo list
   ```

2. Watch as:
   - MOE Router selects the best provider (Abacus AI)
   - Streaming response appears in real-time
   - Code is generated with syntax highlighting

3. Try different tasks:
   ```
   User: Analyze this data: [1, 2, 3, 4, 5]
   User: Explain how async/await works
   User: Create a REST API endpoint for users
   ```

### 4. Explore SONA Workflows

1. Navigate to **http://localhost:3000/sona**
2. View available workflow templates:
   - Paper-to-Code
   - Text-to-Web
   - Doc-to-API
3. Click **"Run Example"** to see multi-agent orchestration

### 5. Try DeepCode

1. Navigate to **http://localhost:3000/deepcode**
2. Select a workflow:
   - **Paper2Code**: Convert research papers to implementation
   - **Text2Web**: Generate web apps from descriptions
   - **Doc2API**: Build APIs from documentation
3. Upload a file or provide a URL
4. Watch agents collaborate to generate code

---

## Key Features

### 🤖 AI Chat with MOE Router

**Intelligent routing** to the best provider for your task:

```typescript
// Automatic routing based on task type
"Write code" → Abacus AI (best for coding)
"Analyze data" → Claude CLI (best for analysis)
"Simple completion" → GitHub Copilot (fastest)
```

**Usage**:
1. Type your request in chat
2. MOE Router analyzes task
3. Selects optimal provider
4. Streams response in real-time

### 🔄 SONA Multi-Agent Workflows

**Orchestrate multiple agents** for complex tasks:

```typescript
// Example: Paper-to-Code workflow
Document Analyzer → Reads paper → Extracts algorithms
         ↓
Code Generator → Implements code → Generates tests
         ↓
Code Reviewer → Reviews code → Suggests improvements
```

**Execution Strategies**:
- ➡️ **Sequential**: One agent at a time
- ⇄ **Parallel**: Multiple agents simultaneously
- 🔀 **Conditional**: Branch based on results
- 🔁 **Loop**: Repeat until condition met
- 📊 **Map-Reduce**: Distribute and aggregate

### 💻 Local Inference (Optional)

**Run AI models offline** with Qwen3-1.7B:

```bash
# Terminal 1: Start Rust inference server
cd rust_backend/inference_server
cargo run --release

# Terminal 2: Start Next.js (will auto-detect local server)
cd nextjs_space
yarn dev
```

**Benefits**:
- ✅ Fully offline operation
- ✅ Privacy (no data leaves device)
- ✅ 32K context window
- ✅ 4x faster than 7B models
- ✅ Only ~1GB model size

### 📦 Widget System

**Dynamic UI components** generated by AI:

```typescript
Supported Widgets:
- 📝 Text blocks (Markdown)
- 📊 Charts (Bar, Line, Pie)
- 📊 Tables (Sortable, Filterable)
- 🖼️ Images (Zoom, Pan, Rotate)
- 🎥 Videos (Custom controls)
- 🌳 Tree views (Hierarchical data)
- 📦 Containers (Grid, Flex, Tabs)
- 📋 Forms (Dynamic validation)
```

### 🔄 Event Sourcing & Replay

**Time-travel debugging** for your application:

1. All actions stored as events
2. Replay any sequence
3. Debug state issues
4. Share reproducible scenarios

**Access**: http://localhost:3000 (Event Replay section)

### 🤝 Real-time Collaboration

**Work together** with JSON-Patch synchronization:

```typescript
// Automatic state sync across users
User A makes change → Generates patch → Broadcast to all
                                           ↓
                               User B receives → Applies patch
```

---

## Common Tasks

### Task 1: Generate a React Component

**Chat Input**:
```
Create a React component for a user profile card with:
- Avatar image
- Name and bio
- Social media links
- Follow button
```

**Result**: Complete TypeScript React component with Tailwind CSS

### Task 2: Run a SONA Workflow

**API Request**:
```typescript
POST /api/sona
{
  "workflowId": "paper-to-code",
  "input": {
    "paperUrl": "https://arxiv.org/abs/1234.5678"
  }
}
```

**Result**: Generated code with tests and documentation

### Task 3: Build a Full-Stack App

**DeepCode Workflow**:
1. Go to http://localhost:3000/deepcode
2. Select **"Text-to-Web"**
3. Input: `Build a todo app with Next.js and MongoDB`
4. Click **"Generate"**
5. Download generated code

### Task 4: Enable Local Inference

**Configuration** (`config/providers.json`):
```json
{
  "ai": {
    "type": "ruvllm",
    "useLocal": true,
    "localUrl": "http://127.0.0.1:8080"
  }
}
```

**Start Server**:
```bash
cd rust_backend/inference_server
cargo run --release
```

### Task 5: Deploy to Production

**Build**:
```bash
yarn build
```

**Deploy** (Choose one):
```bash
# Vercel
vercel deploy

# Docker
docker build -t ml-devops .
docker run -p 3000:3000 ml-devops

# Desktop (Tauri)
yarn tauri build
```

---

## Troubleshooting

### Problem: "Database connection failed"

**Solution**:
```bash
# Check DATABASE_URL in .env
echo $DATABASE_URL

# Test connection
yarn prisma db pull

# Regenerate client
yarn prisma generate
```

### Problem: "API key invalid"

**Solution**:
```bash
# Verify ABACUSAI_API_KEY in .env
cat .env | grep ABACUSAI_API_KEY

# Get new key from:
# https://apps.abacus.ai/settings/api-keys

# Update .env and restart server
yarn dev
```

### Problem: "Local inference not working"

**Solution**:
```bash
# Check if Rust server is running
lsof -i :8080

# Start server manually
cd rust_backend/inference_server
cargo run --release

# Check logs
tail -f rust_backend/inference_server/logs/latest.log
```

### Problem: "Port 3000 already in use"

**Solution**:
```bash
# Find process using port 3000
lsof -ti:3000

# Kill process
kill -9 $(lsof -ti:3000)

# Or use different port
PORT=3001 yarn dev
```

### Problem: "Module not found errors"

**Solution**:
```bash
# Clear cache and reinstall
rm -rf node_modules .next
yarn install

# Regenerate Prisma client
yarn prisma generate

# Restart dev server
yarn dev
```

---

## Next Steps

### 📖 Learn More

1. **Architecture** - [ARCHITECTURE.md](./ARCHITECTURE.md)
2. **Agent System** - [AGENT.md](./AGENT.md)
3. **NOA System** - [NOA_INTEGRATION_FIXED.md](./NOA_INTEGRATION_FIXED.md)
4. **Setup Details** - [SETUP.md](./SETUP.md)

### 🛠️ Advanced Setup

1. **Google SSO** - Configure OAuth authentication
2. **Analytics** - Set up Umami tracking
3. **E2E Tests** - Run Playwright test suite
4. **Mobile Apps** - Build iOS/Android with Tauri

### 🔧 Development

1. **Add Providers** - Integrate new AI providers
2. **Create Agents** - Build custom SONA agents
3. **Custom Widgets** - Design new UI components
4. **Workflows** - Define DeepCode workflows

### 🚀 Deployment

1. **Web Deployment**:
   ```bash
   yarn build
   yarn start
   ```

2. **Desktop App**:
   ```bash
   yarn tauri build
   ```

3. **Docker**:
   ```bash
   docker build -t ml-devops .
   docker run -p 3000:3000 ml-devops
   ```

### 👥 Community

- **Documentation**: http://localhost:3000/docs
- **Issues**: Report bugs and request features
- **Contributing**: See CONTRIBUTING.md (coming soon)
- **Discord**: Join our community (coming soon)

---

## Quick Reference

### Development Commands

```bash
# Start dev server
yarn dev

# Build for production
yarn build

# Start production server
yarn start

# Run tests
yarn test

# Run E2E tests
yarn test:e2e

# Lint code
yarn lint

# Format code
yarn format
```

### Database Commands

```bash
# Generate Prisma client
yarn prisma generate

# Push schema changes
yarn prisma db push

# Create migration
yarn prisma migrate dev

# Open Prisma Studio
yarn prisma studio

# Seed database
yarn prisma db seed
```

### Tauri Commands

```bash
# Start Tauri dev
yarn tauri dev

# Build desktop app
yarn tauri build

# Build for Android
yarn tauri android build

# Build for iOS
yarn tauri ios build
```

### Local Inference

```bash
# Start inference server
cd rust_backend/inference_server
cargo run --release

# Check server health
curl http://localhost:8080/health

# Test completion
curl http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "qwen3",
    "messages": [{"role": "user", "content": "Hello"}]
  }'
```

---

## Configuration Files

### Key Files

```
ml_devops_platform/
├── nextjs_space/
│   ├── .env                    # Environment variables
│   ├── next.config.js          # Next.js configuration
│   ├── tailwind.config.ts      # Tailwind CSS
│   ├── prisma/
│   │   └── schema.prisma       # Database schema
│   ├── config/
│   │   ├── providers.json      # AI providers
│   │   ├── features.json       # Feature flags
│   │   └── ui.json             # UI preferences
│   └── NOA_HOME/               # NOA configuration
│       ├── immutable/          # Schemas, kernels
│       └── mutable/            # Runtime config
└── rust_backend/
    └── inference_server/       # Local AI server
```

### Environment Variables Reference

```bash
# Required
DATABASE_URL="postgresql://..."
ABACUSAI_API_KEY="your-key"
NEXTAUTH_SECRET="generated-secret"
NEXTAUTH_URL="http://localhost:3000"

# Optional - Local Inference
LOCAL_INFERENCE_PORT=8080
LOCAL_INFERENCE_MODEL="Qwen3-1.7B-Instruct"

# Optional - Analytics
NEXT_PUBLIC_UMAMI_WEBSITE_ID="..."
NEXT_PUBLIC_UMAMI_URL="https://..."
NEXT_PUBLIC_UMAMI_DISABLE_LOCALHOST_TRACKING=true

# Optional - Google SSO
GOOGLE_CLIENT_ID="..."
GOOGLE_CLIENT_SECRET="..."

# Optional - Feature Flags
NEXT_PUBLIC_ENABLE_SONA=true
NEXT_PUBLIC_ENABLE_DEEPCODE=true
NEXT_PUBLIC_ENABLE_LOCAL_INFERENCE=true
```

---

## FAQ

### Q: Do I need Rust installed?

**A**: No, Rust is **optional**. It's only needed if you want to run local inference. The platform works perfectly with Abacus AI (cloud) provider.

### Q: Can I use a different database?

**A**: Yes! The platform uses Prisma ORM, which supports PostgreSQL, MySQL, SQLite, SQL Server, MongoDB, and CockroachDB. Update `DATABASE_URL` in `.env`.

### Q: How much does it cost?

**A**: 
- **Abacus AI**: Pay-as-you-go (very affordable)
- **Local Inference**: Free (runs on your machine)
- **Hosting**: Depends on provider (Vercel, AWS, etc.)

### Q: Can I deploy to production?

**A**: Absolutely! The platform is production-ready. See [SETUP.md](./SETUP.md) for deployment guides.

### Q: How do I add a new AI provider?

**A**: See [AGENT.md](./AGENT.md) for detailed instructions on implementing the `AIProvider` interface and integrating with MOE Router.

### Q: Is this suitable for enterprise?

**A**: Yes! Features include:
- ✅ Authentication (NextAuth.js)
- ✅ Authorization (role-based)
- ✅ Audit logging (event sourcing)
- ✅ Analytics (Umami)
- ✅ E2E testing (Playwright)
- ✅ Type safety (TypeScript)

---

## Getting Help

### Resources

- 📖 **Documentation**: http://localhost:3000/docs
- 📝 **Architecture**: [ARCHITECTURE.md](./ARCHITECTURE.md)
- 🤖 **Agents**: [AGENT.md](./AGENT.md)
- ⚙️ **Setup**: [SETUP.md](./SETUP.md)

### Support Channels

- **GitHub Issues**: Report bugs
- **Discord**: Community chat (coming soon)
- **Email**: support@example.com (coming soon)

### Common Issues

- **Database**: Check `DATABASE_URL` is correct
- **API Keys**: Verify `ABACUSAI_API_KEY` is set
- **Ports**: Ensure 3000 and 8080 are available
- **Dependencies**: Run `yarn install` after git pull

---

## Success Checklist

☐ Installed Node.js 18+ and Yarn  
☐ Cloned repository and installed dependencies  
☐ Configured `.env` with database and API keys  
☐ Generated Prisma client and pushed schema  
☐ Started dev server on http://localhost:3000  
☐ Created user account and logged in  
☐ Tried chat with MOE Router  
☐ Explored SONA workflows  
☐ Tested DeepCode (optional)  
☐ Started local inference server (optional)  
☐ Read documentation (ARCHITECTURE, AGENT, etc.)  

---

**🎉 Congratulations!** You're now ready to build with the ML DevOps Platform!

**Next**: Dive into [AGENT.md](./AGENT.md) to learn about the AI provider system and multi-agent orchestration.
