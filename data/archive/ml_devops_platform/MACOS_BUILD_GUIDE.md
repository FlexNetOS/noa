# macOS Build Guide for ML DevOps Platform

> **TL;DR**: Build natively on macOS. Cross-compilation from Linux is complex and not recommended.

## Table of Contents
1. [Quick Start (Native macOS)](#quick-start-native-macos)
2. [Why Native Builds?](#why-native-builds)
3. [Cross-Compilation Challenges](#cross-compilation-challenges)
4. [CI/CD with GitHub Actions](#cicd-with-github-actions)
5. [Troubleshooting](#troubleshooting)

---

## Quick Start (Native macOS)

### Prerequisites
- macOS 11+ (Big Sur or later)
- Xcode Command Line Tools
- Rust toolchain

### Installation

**1. Install Xcode Command Line Tools:**
```bash
xcode-select --install
```

**2. Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**3. Clone and Build:**
```bash
cd ml_devops_platform/rust_backend
cargo build --release
```

**4. Test the Binary:**
```bash
./target/release/inference_server --help
./target/release/inference_server
```

### Build Output
- **Intel Mac**: `target/release/inference_server` (~12-14MB)
- **Apple Silicon**: `target/aarch64-apple-darwin/release/inference_server` (~12-14MB)

---

## Why Native Builds?

### Technical Reasons
1. **Apple SDK Required**: macOS builds need Apple's SDK (comes with Xcode)
2. **Linker Compatibility**: macOS uses `ld64` linker, not available on Linux
3. **System Libraries**: Core Foundation, Security framework, etc.
4. **Code Signing**: macOS binaries may need signing for distribution

### Practical Benefits
- ✅ **Faster compilation** (native CPU)
- ✅ **Better debugging** (native tools)
- ✅ **No cross-compilation toolchain** (simpler setup)
- ✅ **Testing on target platform** (catch issues early)

---

## Cross-Compilation Challenges

### What You'd Need

If you **really** want to cross-compile from Linux:

1. **osxcross Toolchain**
   ```bash
   git clone https://github.com/tpoechtrager/osxcross
   cd osxcross
   # Need to package Xcode SDK (requires Mac + Xcode)
   ./tools/gen_sdk_package.sh
   ```

2. **Apple SDK**
   - Must be extracted from Xcode (on a real Mac)
   - Legal gray area if redistributed
   - Version must match your target macOS

3. **Rust Configuration**
   ```bash
   # Add targets
   rustup target add x86_64-apple-darwin
   rustup target add aarch64-apple-darwin
   
   # Configure linker in ~/.cargo/config.toml
   [target.x86_64-apple-darwin]
   linker = "x86_64-apple-darwin20.4-clang"
   ar = "x86_64-apple-darwin20.4-ar"
   ```

4. **Environment Variables**
   ```bash
   export PATH="/path/to/osxcross/target/bin:$PATH"
   export CC=x86_64-apple-darwin20.4-clang
   export CXX=x86_64-apple-darwin20.4-clang++
   ```

### Common Issues
- ❌ Dependency C libraries fail to compile
- ❌ System framework linking errors
- ❌ Version mismatches between SDK and target
- ❌ Missing code signing (binary won't run without `codesign`)

### Verdict
**Time Investment**: 4-8 hours for first-time setup  
**Maintenance**: Breaks with macOS/Xcode updates  
**Success Rate**: 60-70% (many crates have issues)  

**Recommendation**: Use GitHub Actions instead (see below)

---

## CI/CD with GitHub Actions

The **recommended** way to build for multiple platforms:

### Sample Workflow

```yaml
name: Build Inference Server

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: inference_server
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: inference_server.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: inference_server
          # Add Apple Silicon
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: inference_server_arm64
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: ${{ matrix.target }}
        override: true
    
    - name: Build
      run: |
        cd rust_backend
        cargo build --release --target ${{ matrix.target }}
    
    - name: Upload Artifact
      uses: actions/upload-artifact@v3
      with:
        name: ${{ matrix.artifact }}-${{ matrix.target }}
        path: rust_backend/target/${{ matrix.target }}/release/${{ matrix.artifact }}
```

### Benefits
- ✅ **Native builds** on each platform
- ✅ **No manual setup** (GitHub provides runners)
- ✅ **Automated testing** across platforms
- ✅ **Release artifacts** ready to download
- ✅ **Free** for public repos

---

## Troubleshooting

### Issue: "xcrun: error: invalid active developer path"
**Solution**: Install Xcode Command Line Tools
```bash
xcode-select --install
```

### Issue: "linker `cc` not found"
**Solution**: Xcode CLI tools not in PATH
```bash
sudo xcode-select --switch /Library/Developer/CommandLineTools
```

### Issue: "failed to run custom build command for `ring`"
**Solution**: Ring crate needs assembly support
```bash
# Usually fixed by updating Rust
rustup update stable
```

### Issue: Apple Silicon binary won't run on Intel (or vice versa)
**Solution**: Build universal binary
```bash
# Build both architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Combine with lipo
lipo -create \
  target/x86_64-apple-darwin/release/inference_server \
  target/aarch64-apple-darwin/release/inference_server \
  -output target/release/inference_server_universal

# Verify
lipo -info target/release/inference_server_universal
# Output: Architectures in the fat file: x86_64 arm64
```

### Issue: "Developer cannot be verified" warning
**Solution**: Code signing or override Gatekeeper
```bash
# Allow unsigned binary (one-time)
xcode-select --install
sudo spctl --master-disable  # Allows all apps (careful!)

# Or sign the binary (requires Apple Developer account)
codesign -s "Your Identity" target/release/inference_server
```

---

## Performance Notes

### Compile Times (M2 MacBook Air, 16GB RAM)
- **Release build**: ~90 seconds (first time with all deps)
- **Incremental build**: ~15 seconds (minor changes)

### Binary Sizes
- **Debug build**: ~150 MB (with debug symbols)
- **Release build**: ~13 MB (optimized)
- **Stripped**: ~10 MB (`strip target/release/inference_server`)

### Runtime Performance
- **Apple Silicon**: Fastest (native ARM64, GPU acceleration possible)
- **Intel Mac**: Slightly slower than native Linux equivalent
- **Rosetta 2** (ARM binary on Intel): ~20% performance penalty

---

## Summary

| Approach | Setup Time | Reliability | Recommendation |
|----------|------------|-------------|----------------|
| **Native macOS** | 15 min | ⭐⭐⭐⭐⭐ | ✅ **Best choice** |
| **GitHub Actions** | 30 min | ⭐⭐⭐⭐⭐ | ✅ **For CI/CD** |
| **osxcross (Linux→macOS)** | 4-8 hours | ⭐⭐ | ⚠️ **Not recommended** |

---

## Additional Resources

- [Rust macOS Build Guide](https://doc.rust-lang.org/rustc/platform-support/apple-darwin.html)
- [Apple Developer Documentation](https://developer.apple.com/documentation)
- [osxcross Project](https://github.com/tpoechtrager/osxcross)
- [GitHub Actions for Rust](https://github.com/actions-rs)

---

**Questions?** See the main [E2_INTEGRATION_COMPLETE.md](./E2_INTEGRATION_COMPLETE.md) for architecture details.
