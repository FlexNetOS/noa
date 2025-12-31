# Build Guide - ML DevOps Platform

This guide provides instructions for building the ML DevOps Platform for various platforms including macOS, Windows, Linux, Android, and iOS.

## Prerequisites

### All Platforms
- **Node.js** (v18 or later)
- **Yarn** package manager
- **Rust** toolchain (rustc, cargo)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Platform-Specific Requirements

#### macOS
- **Xcode** (latest version from App Store)
- **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```

#### Windows
- **Visual Studio** (with C++ build tools)
- **WebView2** (usually pre-installed on Windows 10/11)

#### Linux
- **Development libraries**:
  ```bash
  # Debian/Ubuntu
  sudo apt update
  sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

  # Fedora
  sudo dnf install webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel

  # Arch Linux
  sudo pacman -Syu
  sudo pacman -S webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libvips
  ```

#### Android
- **Android Studio** (latest version)
- **Android SDK** (API level 24+)
- **Android NDK** (r25c or later)
- **Java Development Kit** (JDK 17 or later)
- Set environment variables:
  ```bash
  export ANDROID_HOME=$HOME/Android/Sdk
  export NDK_HOME=$ANDROID_HOME/ndk/<version>
  export PATH=$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools
  ```
- Install Rust Android targets:
  ```bash
  rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
  ```

#### iOS (macOS only)
- **Xcode** (latest version)
- **iOS SDK** (included with Xcode)
- **CocoaPods**:
  ```bash
  sudo gem install cocoapods
  ```
- Install Rust iOS targets:
  ```bash
  rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
  ```

## Setup

1. **Clone the repository** (or navigate to the project directory):
   ```bash
   cd ml_devops_platform/nextjs_space
   ```

2. **Install dependencies**:
   ```bash
   yarn install
   ```

3. **Set up environment variables**:
   - Copy `.env.example` to `.env`
   - Fill in required values (DATABASE_URL, API keys, etc.)

4. **Initialize the database**:
   ```bash
   yarn prisma db push
   yarn prisma generate
   ```

## Building

### Desktop Builds

#### Development Mode
Run the app in development mode with hot-reload:

```bash
cd nextjs_space
yarn tauri:dev
```

#### Production Builds

**macOS**:
```bash
cd nextjs_space
yarn tauri:build
```
Outputs:
- DMG installer: `src-tauri/target/release/bundle/dmg/`
- App bundle: `src-tauri/target/release/bundle/macos/ML DevOps Platform.app`

**Windows**:
```bash
cd nextjs_space
yarn tauri:build
```
Outputs:
- MSI installer: `src-tauri/target/release/bundle/msi/`
- EXE: `src-tauri/target/release/ML DevOps Platform.exe`

**Linux**:
```bash
cd nextjs_space
yarn tauri:build
```
Outputs:
- AppImage: `src-tauri/target/release/bundle/appimage/`
- Debian package: `src-tauri/target/release/bundle/deb/`

### Mobile Builds

#### Android

1. **Initialize Android project** (first time only):
   ```bash
   cd nextjs_space
   yarn tauri android init
   ```

2. **Development mode**:
   ```bash
   yarn tauri:android
   ```
   This will launch the app in an Android emulator or connected device.

3. **Production build**:
   ```bash
   yarn tauri:android:build
   ```
   Output: `src-tauri/gen/android/app/build/outputs/apk/`

4. **Generate signed APK** (for distribution):
   - Open `src-tauri/gen/android` in Android Studio
   - Build > Generate Signed Bundle/APK
   - Follow the wizard to create a keystore and sign the APK

#### iOS (macOS only)

1. **Initialize iOS project** (first time only):
   ```bash
   cd nextjs_space
   yarn tauri ios init
   ```

2. **Development mode**:
   ```bash
   yarn tauri:ios
   ```
   This will launch the app in the iOS Simulator.

3. **Production build**:
   ```bash
   yarn tauri:ios:build
   ```
   Output: `src-tauri/gen/ios/`

4. **Deploy to App Store**:
   - Open `src-tauri/gen/ios/ML DevOps Platform.xcodeproj` in Xcode
   - Configure signing & capabilities
   - Select Generic iOS Device as the destination
   - Product > Archive
   - Follow the App Store submission process

## Quick Start with Automated Scripts

For a streamlined build process, use the provided automation scripts:

### Environment Setup
```bash
cd nextjs_space
./scripts/setup-env.sh
```

This script will:
- Check and install Node.js, Yarn, and Rust
- Install project dependencies
- Set up environment variables
- Initialize the database
- Optionally install Playwright browsers

### Desktop Build
```bash
cd nextjs_space
./scripts/build-desktop.sh
```

Automatically detects your platform (macOS, Windows, Linux) and builds the appropriate installer.

### Mobile Build
```bash
cd nextjs_space
# Build both Android and iOS
./scripts/build-mobile.sh all

# Build only Android
./scripts/build-mobile.sh android

# Build only iOS (macOS only)
./scripts/build-mobile.sh ios
```

## Platform-Specific Configuration

### Google SSO Setup

The app includes **Dynamic Google Single Sign-On** with two setup options:

#### Option 1: Admin UI Configuration (Recommended)

1. Start the development server or deploy the app
2. Sign in as an administrator
3. Navigate to `/admin/oauth-setup`
4. Follow the setup wizard to configure Google OAuth
5. OAuth credentials are encrypted and stored securely in the database
6. No server restart required - changes take effect immediately

**Benefits:**
- No environment file editing required
- Secure encrypted storage
- Real-time updates without deployment
- User-friendly interface with step-by-step guidance

#### Option 2: Environment Variables (Traditional)

Alternatively, you can still use environment variables for OAuth configuration:

1. **Create OAuth credentials**:
   - Go to [Google Cloud Console](https://console.cloud.google.com)
   - Create a new project or select existing
   - Enable Google+ API
   - Create OAuth 2.0 credentials

2. **Configure authorized URIs**:
   - **For web/desktop**: `http://localhost:3000/api/auth/callback/google`
   - **For production**: `https://your-domain.com/api/auth/callback/google`
   - **For mobile**: Use custom URL scheme (e.g., `ai.mldevops.platform://oauth/callback`)

3. **Update environment variables**:
   ```bash
   GOOGLE_CLIENT_ID=your_client_id_here
   GOOGLE_CLIENT_SECRET=your_client_secret_here
   ```

### Database Configuration

#### Desktop Apps
Desktop builds use the same PostgreSQL database as the web version. Ensure `DATABASE_URL` is set correctly in `.env`.

#### Mobile Apps
For mobile, you may want to use:
- **SQLite** for offline-first functionality
- **Remote PostgreSQL** for cloud sync

Update `prisma/schema.prisma` accordingly:
```prisma
// For SQLite (mobile)
datasource db {
  provider = "sqlite"
  url      = "file:./dev.db"
}

// For PostgreSQL (web/desktop)
datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}
```

### Rust Inference Server

The app includes a local Rust inference server for ML operations:

```bash
cd rust_backend/inference_server
cargo build --release

# Run the server
./target/release/inference_server
```

For cross-compilation (e.g., building macOS binary on Linux):
```bash
# Install cross-compilation tools
cargo install cross

# Build for macOS (from Linux)
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin  # Apple Silicon

# Build for Windows (from Linux/macOS)
cross build --release --target x86_64-pc-windows-gnu
```

## Troubleshooting

### Build Errors

**"cargo not found"**:
- Install Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Restart terminal to load environment

**"failed to run 'tauri' command"**:
- Ensure Tauri CLI is installed: `yarn install`
- Check Rust toolchain: `rustc --version`

**Android build fails**:
- Verify ANDROID_HOME and NDK_HOME are set
- Check Android SDK and NDK are installed
- Ensure Java 17+ is installed: `java -version`

**iOS build fails**:
- Update Xcode to latest version
- Install Command Line Tools: `xcode-select --install`
- Accept Xcode license: `sudo xcodebuild -license accept`

### Runtime Issues

**Database connection errors**:
- Check `DATABASE_URL` in `.env`
- Ensure PostgreSQL is running
- Verify database migrations: `yarn prisma db push`

**Google SSO not working**:
- Verify OAuth credentials in `.env`
- Check authorized redirect URIs in Google Cloud Console
- Ensure `NEXTAUTH_SECRET` is set

**Inference server not starting**:
- Check if port 8080 is available
- Verify Rust server binary is built: `cd rust_backend/inference_server && cargo build --release`
- Check logs for model download issues

## Distribution

### Desktop Apps
- **macOS**: Distribute DMG file, optionally notarize for Gatekeeper
- **Windows**: Distribute MSI installer, optionally code-sign
- **Linux**: Distribute AppImage (universal) or DEB/RPM packages

### Mobile Apps
- **Android**: Upload APK/AAB to Google Play Store
- **iOS**: Submit via Xcode to App Store Connect

## Additional Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Next.js Documentation](https://nextjs.org/docs)
- [Prisma Documentation](https://www.prisma.io/docs)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)

## Support

For issues or questions:
- Check existing documentation in `ARCHITECTURE.md`, `SETUP.md`
- Review Tauri troubleshooting guides
- Check platform-specific build logs

---

**Note**: Mobile builds are experimental and may require additional configuration. Desktop builds (macOS, Windows, Linux) are production-ready.
