# Portable configsuration

This directory contains **portable configsuration** that is:
- ✅ **Git-tracked** (safe to commit)
- ✅ **Project-specific** (applies to all users)
- ✅ **Cross-platform** (works on Windows/Mac/Linux)
- ❌ **NO SECRETS** (never put API keys here)

## configsuration Files

### `app.json`
Application metadata and environment settings.

### `features.json`
Feature flags to enable/disable functionality.

### `providers.json`
AI provider configsuration (without API keys).

### `ui.json`
UI preferences and theme settings.

## Override Priority

configsuration is loaded in this order (highest priority first):

1. **Environment variables** (`.env` file)
   - `DATABASE_URL`
   - `ABACUSAI_API_KEY`
   - `NODE_ENV`
   - `LOG_LEVEL`

2. **Installed configs** (user-specific)
   - Windows: `%APPDATA%/ml-devops/configs.json`
   - macOS: `Library/Application Support/ml-devops/configs.json` (in user home)
   - Linux: `.configs/ml-devops/configs.json` (in user home)

3. **Portable configs** (this directory)
   - `./configs/*.json`

4. **Defaults** (hardcoded in `lib/configs/validator.ts`)

## User Overrides

To create user-specific overrides:

```bash
# Create installed configs directory (Linux/macOS)
mkdir -p $HOME/.configs/ml-devops

# Create configs file
cat > $HOME/.configs/ml-devops/configs.json << EOF
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
2. `.configs/ml-devops/secrets.json` in user home (installed configs) - **fallback**

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
# configs/app.toml
[app]
name = "ML DevOps Platform"
version = "1.0.0"
environment = "development"
```

The configs loader will use the `toml` crate and `serde` for deserialization.
