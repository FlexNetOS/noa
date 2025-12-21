# Tauri Desktop Setup Guide

This guide will help you set up and run the ML DevOps Platform as a native desktop application using Tauri v2.

## 📋 Prerequisites

### Required Dependencies

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Node.js** (v18 or later) - Already installed

3. **System Dependencies**

   **Linux (Ubuntu/Debian)**:
   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

   **macOS**:
   ```bash
   # Xcode Command Line Tools
   xcode-select --install
   ```

   **Windows**:
   - Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 11)

## 🚀 Quick Start

### Development Mode

Run the desktop app in development mode:

```bash
cd nextjs_space
yarn tauri dev
```

This will:
1. Start the Next.js dev server on `http://localhost:3000`
2. Compile the Rust backend
3. Launch the native desktop window

**Note**: The first compilation will take several minutes as Cargo downloads and compiles dependencies.

### Production Build

Build installers for your platform:

```bash
cd nextjs_space
yarn tauri build
```

Built artifacts will be in `src-tauri/target/release/bundle/`:

- **Linux**: `.deb`, `.appimage`, `.rpm`
- **macOS**: `.dmg`, `.app`
- **Windows**: `.msi`, `.exe`

### Debug Build

For faster builds during testing:

```bash
yarn tauri build --debug
```

## 🏗️ Architecture

### Project Structure

```
ml_devops_platform/
├── nextjs_space/
│   ├── app/                 # Next.js frontend
│   ├── components/          # React components
│   ├── lib/                 # Utilities and business logic
│   │   └── tauri/           # Tauri command bindings
│   │       └── commands.ts  # TypeScript API for Tauri
│   ├── src-tauri/           # Tauri Rust backend
│   │   ├── src/
│   │   │   ├── lib.rs       # Main application logic
│   │   │   └── main.rs      # Entry point
│   │   ├── Cargo.toml       # Rust dependencies
│   │   ├── tauri.conf.json  # Tauri configuration
│   │   ├── capabilities/    # Permission definitions
│   │   └── icons/           # Application icons
│   └── package.json
└── TAURI_SETUP.md
```

### How It Works

1. **Frontend**: Next.js app runs in development mode or is built
2. **Backend**: Rust binary wraps the frontend in a native window
3. **Communication**: TypeScript ↔ Rust via Tauri's IPC bridge
4. **Database**: Shared PostgreSQL instance (local or remote)

## 🛠️ Available Tauri Commands

The platform exposes several Rust commands that can be called from TypeScript:

### System Information
```typescript
import { getSystemInfo, isTauriContext } from '@/lib/tauri/commands';

if (isTauriContext()) {
  const info = await getSystemInfo();
  console.log('Platform:', info.platform);
  console.log('Architecture:', info.arch);
}
```

### File Operations
```typescript
import { saveLocalFile, readLocalFile } from '@/lib/tauri/commands';

// Save file
await saveLocalFile('/path/to/file.txt', 'contents');

// Read file
const contents = await readLocalFile('/path/to/file.txt');
```

### Application Info
```typescript
import { isDesktopMode, getAppVersion } from '@/lib/tauri/commands';

const isDesktop = await isDesktopMode(); // true in Tauri
const version = await getAppVersion(); // "0.1.0"
```

### Future: Local ML Inference
```typescript
import { 
  startInferenceServer, 
  stopInferenceServer, 
  getInferenceStatus 
} from '@/lib/tauri/commands';

// These are placeholders for Phase E implementation
const serverUrl = await startInferenceServer();
const isRunning = await getInferenceStatus();
await stopInferenceServer();
```

## 📦 Building for Distribution

### Linux

**AppImage** (Universal):
```bash
yarn tauri build --bundles appimage
```

**Debian Package**:
```bash
yarn tauri build --bundles deb
```

**RPM Package**:
```bash
yarn tauri build --bundles rpm
```

### macOS

**DMG Installer**:
```bash
yarn tauri build --bundles dmg
```

**App Bundle**:
```bash
yarn tauri build --bundles app
```

**Note**: For distribution outside the App Store, you'll need to sign the app with a Developer ID certificate.

### Windows

**MSI Installer**:
```bash
yarn tauri build --bundles msi
```

**NSIS Installer**:
```bash
yarn tauri build --bundles nsis
```

## 🔧 Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)

Key configuration options:

```json
{
  "productName": "ML DevOps Platform",
  "identifier": "com.mldevops.platform",
  "version": "0.1.0",
  "build": {
    "devUrl": "http://localhost:3000",
    "frontendDist": "../.next"
  },
  "app": {
    "windows": [
      {
        "title": "ML DevOps Platform",
        "width": 1400,
        "height": 900,
        "minWidth": 800,
        "minHeight": 600
      }
    ]
  }
}
```

### Environment Variables

Create `nextjs_space/.env.desktop` for desktop-specific configuration:

```bash
# Database (use local for desktop)
DATABASE_URL="postgresql://localhost:5432/mldevops_desktop"

# Disable web analytics in desktop mode
NEXT_PUBLIC_UMAMI_WEBSITE_ID=""

# Desktop-specific features
TAURI_ENV="desktop"
```

## 🐛 Troubleshooting

### Build Fails

**Issue**: Rust dependencies fail to compile

**Solution**:
```bash
# Clean Rust build cache
cd src-tauri
cargo clean

# Update Rust
rustup update

# Rebuild
cd ..
yarn tauri build
```

### Window Doesn't Open

**Issue**: Desktop window fails to launch

**Solution**: Check the logs in `src-tauri/target/debug/` or `src-tauri/target/release/`

### Database Connection Errors

**Issue**: Can't connect to PostgreSQL

**Solution**: 
1. Ensure PostgreSQL is running: `systemctl status postgresql`
2. Update `DATABASE_URL` in `.env`
3. Create the database: `createdb mldevops_desktop`
4. Run migrations: `yarn prisma migrate dev`

### WebView2 Missing (Windows)

**Issue**: Error about missing WebView2

**Solution**: Download and install [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

## 🎯 Next Steps

### Current Status (Phase D Complete)

✅ Tauri v2 project structure
✅ Rust backend with system integration commands
✅ TypeScript bindings for Tauri commands
✅ Desktop window configuration
✅ Build scripts for all platforms
✅ Plugin integration (fs, dialog, shell, http, websocket)

### Future Phases

**Phase E: Rust Backend + Local Inference** (~3,000 credits)
- Implement Candle-vLLM integration
- Add local model serving (Qwen2.5-7B)
- HTTP API server with axum
- Benchmark local vs cloud inference
- Offline mode support

**Phase F: Advanced Desktop Features**
- Auto-updates via Tauri updater
- System tray integration
- Native notifications
- Keyboard shortcuts
- Multi-window support

## 📚 Resources

- [Tauri Documentation](https://v2.tauri.app/)
- [Next.js with Tauri Guide](https://v2.tauri.app/start/create-project/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Candle ML Framework](https://github.com/huggingface/candle)

## 🆘 Support

For issues and questions:

1. Check [Tauri GitHub Issues](https://github.com/tauri-apps/tauri/issues)
2. Review [Next.js Discussions](https://github.com/vercel/next.js/discussions)
3. Consult project documentation in `/nextjs_space/README.md`

---

**Built with ❤️ using Tauri v2, Next.js 14, React 18, TypeScript, and Rust**
