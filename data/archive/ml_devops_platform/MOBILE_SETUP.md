# Mobile Setup Guide (iOS & Android)

This guide covers building and deploying the ML DevOps Platform on mobile devices using Tauri Mobile.

## Prerequisites

### iOS Development
- macOS with Xcode 15+ installed
- Apple Developer account (for device testing/distribution)
- iOS 13.0+ target device or simulator
- CocoaPods: `sudo gem install cocoapods`

### Android Development
- Android Studio with SDK 34+
- Android NDK
- JDK 17+
- Min SDK: Android 7.0 (API 24)

### Rust Toolchain
```bash
# Install mobile targets
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

## Quick Start

### 1. Install Tauri CLI with Mobile Support
```bash
cargo install tauri-cli --version "^2.0.0"
```

### 2. Initialize Mobile Projects
```bash
cd nextjs_space

# Initialize iOS
cargo tauri ios init

# Initialize Android
cargo tauri android init
```

### 3. Development

#### iOS Simulator
```bash
cargo tauri ios dev
```

#### Android Emulator
```bash
cargo tauri android dev
```

## Building for Release

### iOS
```bash
# Build for App Store
cargo tauri ios build --release

# The .ipa file will be in:
# src-tauri/gen/apple/build/arm64/release/bundle/ios/
```

### Android
```bash
# Build APK
cargo tauri android build --release

# Build AAB (for Play Store)
cargo tauri android build --release --aab

# Output locations:
# APK: src-tauri/gen/android/app/build/outputs/apk/
# AAB: src-tauri/gen/android/app/build/outputs/bundle/
```

## Platform-Specific Features

### Mobile Capabilities
| Feature | iOS | Android | Notes |
|---------|-----|---------|-------|
| Cloud AI Providers | ✅ | ✅ | All providers work |
| Local Inference | ⚠️ | ⚠️ | Limited by device |
| Push Notifications | ✅ | ✅ | Requires setup |
| Biometric Auth | ✅ | ✅ | Face ID / Fingerprint |
| Offline Mode | ✅ | ✅ | Cached responses |
| File Upload | ✅ | ✅ | Photo library access |

### Provider Routing
On mobile, the MOE router automatically:
1. Prefers cloud providers (Abacus AI, GitHub Copilot)
2. Falls back to cached responses when offline
3. Disables local inference by default (resource constraints)

## Configuration

### iOS Info.plist Additions
```xml
<key>NSCameraUsageDescription</key>
<string>Used for document scanning</string>
<key>NSPhotoLibraryUsageDescription</key>
<string>Used for uploading images</string>
<key>NSFaceIDUsageDescription</key>
<string>Used for secure authentication</string>
```

### Android Permissions (AndroidManifest.xml)
```xml
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.CAMERA" />
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
<uses-permission android:name="android.permission.USE_BIOMETRIC" />
```

## Responsive Design

The app uses responsive hooks for optimal mobile experience:

```typescript
import { usePlatformOptimized } from '@/lib/hooks/use-platform';

function MyComponent() {
  const { 
    isMobile, 
    layout, 
    shouldUseCompactUI,
    hasTouch 
  } = usePlatformOptimized();

  return (
    <div style={{ padding: layout.padding }}>
      {shouldUseCompactUI ? <CompactView /> : <FullView />}
    </div>
  );
}
```

## Safe Area Handling

For notched devices (iPhone X+, etc.):

```css
/* globals.css */
:root {
  --sat: env(safe-area-inset-top);
  --sab: env(safe-area-inset-bottom);
  --sal: env(safe-area-inset-left);
  --sar: env(safe-area-inset-right);
}

.safe-area-padding {
  padding-top: max(16px, var(--sat));
  padding-bottom: max(16px, var(--sab));
}
```

## Troubleshooting

### iOS: "Signing requires a development team"
1. Open `src-tauri/gen/apple/` in Xcode
2. Select the target → Signing & Capabilities
3. Add your Apple Developer Team

### Android: "NDK not found"
```bash
# Set NDK path
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653
```

### Build fails with memory error
Mobile builds are resource-intensive:
```bash
# Increase Node memory
export NODE_OPTIONS="--max-old-space-size=8192"
```

## Testing

### iOS Simulator
```bash
# List available simulators
xcrun simctl list devices

# Run on specific simulator
cargo tauri ios dev --device "iPhone 15 Pro"
```

### Android Emulator
```bash
# List available AVDs
emulator -list-avds

# Run on specific emulator
cargo tauri android dev --device "Pixel_7_API_34"
```

## Distribution

### iOS App Store
1. Archive in Xcode: Product → Archive
2. Validate and upload via Xcode Organizer
3. Complete App Store Connect submission

### Google Play Store
1. Build signed AAB
2. Upload to Play Console
3. Complete store listing

### Ad-Hoc / TestFlight
- iOS: Use TestFlight for beta distribution
- Android: Distribute APK directly or use Firebase App Distribution
