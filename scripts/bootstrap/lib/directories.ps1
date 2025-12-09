<#
.SYNOPSIS
    Directory structure creation for NOA bootstrap

.DESCRIPTION
    Creates the complete NOA directory structure within noa_root.
    Per NOA Constitution §3.1: Self-contained installation.
#>

function New-NoaDirectoryStructure {
    <#
    .SYNOPSIS
        Create the complete NOA directory structure

    .PARAMETER NoaRoot
        Root directory for NOA installation

    .PARAMETER Quiet
        Suppress output messages
    #>
    param(
        [Parameter(Mandatory)]
        [string]$NoaRoot,

        [switch]$Quiet
    )

    # Core directories
    $directories = @(
        # Binaries and tools
        "bin",                      # Portable binaries and symlinks
        "opt",                      # Portable toolchains (rust, go, node, python)
        "opt/rust/rustup",          # Rust toolchain
        "opt/rust/cargo",           # Cargo home
        "opt/go",                   # Go installation
        "opt/go/workspace",         # GOPATH workspace
        "opt/go/pkg/mod",           # Go module cache
        "opt/node",                 # Node.js installation
        "opt/python",               # Python installation
        "opt/venv",                 # Python virtual environment
        "opt/protobuf",             # protoc includes
        "opt/dev-tools",            # Cursor, VSCode, etc.
        "opt/npm-cache",            # npm cache
        "opt/cache",                # Download cache

        # Libraries
        "lib",                      # Shared libraries
        "lib/shared",               # Shared .so/.dll files

        # Configuration
        "config",                   # Configuration files
        "config/providers",         # Provider configurations
        "config/i18n",              # Internationalization

        # Cache and temporary
        "cache",                    # General cache
        "cache/rust",               # Rust cache symlink target
        "cache/go",                 # Go cache symlink target
        "cache/npm",                # npm cache symlink target
        "cache/pip",                # pip cache

        # Logs
        "logs",                     # All logs

        # Temporary files
        "tmp",                      # Temporary files

        # P2P network
        "p2p",                      # P2P root
        "p2p/compute",              # P2P compute pool
        "p2p/network",              # P2P network config
        "p2p/storage",              # P2P distributed storage
        "p2p/nodes",                # Known nodes

        # AI system
        "ai",                       # AI root
        "ai/providers",             # AI provider configurations
        "ai/providers/local",       # Local providers (llama.cpp)
        "ai/providers/cloud",       # Cloud providers (claude, codex)
        "ai/providers/hybrid",      # Hybrid providers (cursor)
        "ai/providers/ide",         # IDE providers (copilot)
        "ai/devices",               # Device-specific AI config
        "ai/shared",                # Shared AI resources
        "ai/shared/agents",         # Agent definitions
        "ai/shared/workflows",      # Workflow definitions
        "ai/shared/prompts",        # Prompt templates
        "ai/shared/skills",         # Skill definitions
        "ai/shared/tools",          # MCP tools
        "ai/shared/models",         # Model adapters
        "ai/shared/models/multimodal", # Multi-modal models
        "ai/shared/commands",       # Command definitions
        "ai/shared/resources",      # Shared execution resources

        # System
        "sys",                      # System components
        "sys/core",                 # Core Rust modules
        "sys/kernel",               # Kernel abstraction
        "sys/kernel/modules",       # Kernel modules
        "sys/kernel/params",        # Kernel parameters
        "sys/namespace",            # Process namespaces

        # Git/Version control
        "git",                      # Git data

        # Init and services
        "init",                     # Init system
        "init/run",                 # Runtime data
        "init/services",            # Service definitions

        # Workspace
        "workspace",                # User workspace

        # Containers
        "containers",               # Container data

        # Repositories
        "repos"                     # Cloned repositories
    )

    $created = @()
    $existed = @()

    foreach ($dir in $directories) {
        $fullPath = Join-Path $NoaRoot $dir
        if (-not (Test-Path $fullPath)) {
            New-Item -ItemType Directory -Path $fullPath -Force | Out-Null
            $created += $dir
            if (-not $Quiet) {
                Write-Host "[+] Created: $dir" -ForegroundColor Green
            }
        } else {
            $existed += $dir
        }
    }

    return @{
        Created = $created
        Existed = $existed
        Total = $directories.Count
    }
}

function New-GitIgnoreEntries {
    <#
    .SYNOPSIS
        Get .gitignore entries for NOA directories
    #>

    return @"
# NOA Bootstrap - Auto-generated entries
# Dev tools (installed by bootstrap)
opt/dev-tools/
opt/cursor-cli/
opt/claude-code/
opt/codex/

# Toolchain installations
opt/rust/
opt/go/
opt/node/
opt/python/
opt/venv/
opt/protobuf/
opt/npm-cache/
opt/cache/

# Caches
cache/
tmp/

# Logs (except structure)
logs/*.log
logs/**/*.log

# Runtime state
init/run/
config/bootstrap-state.json

# P2P node data (sensitive)
p2p/nodes/*.json

# AI execution memory
ai/shared/resources/*.db

# Environment files (may contain secrets)
noa-env.ps1
noa-env.sh
.env
.env.local

# IDE specific
.idea/
.vscode/settings.json
.vscode/launch.json
*.swp
*.swo
*~
"@
}

function Update-GitIgnore {
    <#
    .SYNOPSIS
        Update .gitignore with NOA-specific entries

    .PARAMETER NoaRoot
        Root directory for NOA installation
    #>
    param(
        [Parameter(Mandatory)]
        [string]$NoaRoot
    )

    $gitignorePath = Join-Path $NoaRoot ".gitignore"
    $noaEntries = New-GitIgnoreEntries
    $marker = "# NOA Bootstrap - Auto-generated entries"

    if (Test-Path $gitignorePath) {
        $content = Get-Content $gitignorePath -Raw

        if ($content -notmatch [regex]::Escape($marker)) {
            # Add NOA entries
            $content = $content.TrimEnd() + "`n`n$noaEntries"
            Set-Content -Path $gitignorePath -Value $content -Encoding UTF8
            return @{ Action = "updated"; Added = $true }
        } else {
            return @{ Action = "skipped"; Reason = "NOA entries already present" }
        }
    } else {
        Set-Content -Path $gitignorePath -Value $noaEntries -Encoding UTF8
        return @{ Action = "created"; Added = $true }
    }
}

# Export functions
Export-ModuleMember -Function @(
    'New-NoaDirectoryStructure',
    'New-GitIgnoreEntries',
    'Update-GitIgnore'
)

