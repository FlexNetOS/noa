# Portable Configuration

This directory contains **portable configuration** that is:
- ✅ **Git-tracked** (safe to commit)
- ✅ **Project-specific** (applies to all users)
- ✅ **Cross-platform** (works on Windows/Mac/Linux)
- ❌ **NO SECRETS** (never put API keys here)

## Configuration Files

### `app.json`
Application metadata and environment settings.

### `features.json`
Feature flags to enable/disable functionality.

### `providers.json`
AI provider configuration (without API keys).

### `ui.json`
UI preferences and theme settings.

## Override Priority

Configuration is loaded in this order (highest priority first):

1. **Environment variables** (`.env` file)
   - `DATABASE_URL`
   - `ABACUSAI_API_KEY`
   - `NODE_ENV`
   - `LOG_LEVEL`

2. **Installed config** (user-specific)
   - Windows: `%APPDATA%/ml-devops/config.json`
   - macOS: `Library/Application Support/ml-devops/config.json` (in user home)
   - Linux: `.config/ml-devops/config.json` (in user home)

3. **Portable config** (this directory)
   - `./config/*.json`

4. **Defaults** (hardcoded in `lib/config/validator.ts`)

## User Overrides

To create user-specific overrides:

```bash
# Create installed config directory (Linux/macOS)
mkdir -p $HOME/.config/ml-devops

# Create config file
cat > $HOME/.config/ml-devops/config.json << EOF
{
  "ui": {
    "theme": "dark",
    "enableAnimations": false
  },
  "logging": {
    "level": "debug",
    "enableFileLogging": true
  }
}
EOF
```

## Secrets Management

**NEVER put secrets in these files!**

Secrets should be in:
1. `.env` file (environment variables) - **preferred**
2. `.config/ml-devops/secrets.json` in user home (installed config) - **fallback**

Example `secrets.json`:
```json
{
  "abacusApiKey": "your-api-key-here",
  "databaseUrl": "postgresql://..."
}
```

## Rust Migration Notes

In Phase 2, these JSON files will be converted to TOML:

```toml
# config/app.toml
[app]
name = "ML DevOps Platform"
version = "1.0.0"
environment = "development"
```

The config loader will use the `toml` crate and `serde` for deserialization.
