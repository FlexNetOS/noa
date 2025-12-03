# Setup Script Refactoring Instructions

This document describes how the setup script has been refactored for the noa repo.

## Changes Made

The original script `E:\Backups\WSL\setup-agentic-os.ps1` has been adapted for the noa repo with the following changes:

1. **Base Directory**: Changed from `W:\WSL` to `n:\noa`
2. **Script Name**: Changed from `setup-agentic-os.ps1` to `setup-noa.ps1`
3. **Usage Instructions**: Updated to reference `n:\noa\setup-noa.ps1`

## Key Path Changes

- `W:\WSL` → `n:\noa`
- `W:\WSL\images` → `n:\noa\images`
- `W:\WSL\agentic-os` → `n:\noa\agentic-os`
- Script references updated from `setup-agentic-os.ps1` to `setup-noa.ps1`

## Usage

To create the adapted script, run:

```powershell
cd n:\noa
powershell -ExecutionPolicy Bypass -File adapt-script.ps1
```

This will create `setup-noa.ps1` in the `n:\noa` directory.

## Original Script Location

Source: `E:\Backups\WSL\setup-agentic-os.ps1`
Output: `E:\Backups\WSL\agentic-os\home\noa`

## Adapted Script Location

Target: `n:\noa\setup-noa.ps1`
Output: `n:\noa\agentic-os\home\noa` (same structure)

## Structure Preserved

The script maintains the same structure and functionality:
- Creates WSL distro named "agentic-os"
- Sets up users "noa" and "deflex"
- Creates self-contained system in /home/noa
- Installs Docker, Node.js, Python, Ollama, llama.cpp, Gitea
- Configures P2P, AI, Git CI/CD systems
- Same directory structure as original
