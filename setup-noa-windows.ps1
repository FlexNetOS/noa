<#
  Setup script for NOA structure on Windows 11
  Creates the same directory structure that was previously in WSL /home/noa
  Now sets up directly in Windows at N:\noa\

  NOA is a P2P server designed to connect all user devices for dynamically
  shared compute and storage. It's an AI-first, conversational UI/OS that
  prefers CLI tools over IDE extensions.

  Usage (open PowerShell, ideally as Administrator):

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    N:\noa\setup-noa-windows.ps1

  Adjust the variables in the "CONFIGURATION" section if needed.
#>

param(
    [string]$NoaRoot = "N:\noa",
    [switch]$SkipStructure = $false
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "=== Setting up NOA structure at '$NoaRoot' ===" -ForegroundColor Cyan

### Helpers ###################################################################

function Ensure-Dir {
    param([string]$Path)
    try {
        # Ensure we use absolute path
        if (-not [System.IO.Path]::IsPathRooted($Path)) {
            $Path = Join-Path $NoaRoot $Path
        }

        # Use .NET method for more reliable directory creation
        if (-not [System.IO.Directory]::Exists($Path)) {
            Write-Host "Creating directory: $Path" -ForegroundColor Gray
            $null = [System.IO.Directory]::CreateDirectory($Path)
            if ([System.IO.Directory]::Exists($Path)) {
                Write-Host "  ✓ Created: $Path" -ForegroundColor Green
            } else {
                throw "Directory creation failed: $Path"
            }
        } else {
            Write-Host "  ○ Already exists: $Path" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "  ✗ ERROR: Failed to create '$Path': $($_.Exception.Message)" -ForegroundColor Red
        Write-Error "Failed to create directory '$Path': $($_.Exception.Message)"
        throw
    }
}

### 1. Ensure base directory exists ##########################################

Write-Host "Ensuring base directory exists..." -ForegroundColor Cyan
Ensure-Dir -Path $NoaRoot

# Verify we can write to the directory
if (-not (Test-Path $NoaRoot)) {
    throw "Cannot access or create base directory: $NoaRoot"
}

Write-Host "Base directory verified: $NoaRoot" -ForegroundColor Green

### 2. Create NOA monorepo directory structure ###############################

if (-not $SkipStructure) {
    Write-Host "Creating NOA monorepo directory structure..." -ForegroundColor Cyan

    # Main directories
    $mainDirs = @(
        "repos", "containers", "workspace", "config", "scripts",
        "logs", "tmp", "p2p", "ai", "git", "bin", "etc", "lib", "opt", "sys", "init"
    )

    foreach ($dir in $mainDirs) {
        $fullPath = Join-Path $NoaRoot $dir
        Ensure-Dir -Path $fullPath
    }

    # Repos subdirectories
    Write-Host "  Creating repos structure..." -ForegroundColor Gray
    Ensure-Dir -Path (Join-Path $NoaRoot "repos\github")
    Ensure-Dir -Path (Join-Path $NoaRoot "repos\local")
    Ensure-Dir -Path (Join-Path $NoaRoot "repos\external")

    # Container hosting directories
    Write-Host "  Creating containers structure..." -ForegroundColor Gray
    Ensure-Dir -Path (Join-Path $NoaRoot "containers\docker")
    Ensure-Dir -Path (Join-Path $NoaRoot "containers\compose")
    Ensure-Dir -Path (Join-Path $NoaRoot "containers\volumes")

    # Workspace directories
    Write-Host "  Creating workspace structure..." -ForegroundColor Gray
    Ensure-Dir -Path (Join-Path $NoaRoot "workspace\projects")
    Ensure-Dir -Path (Join-Path $NoaRoot "workspace\agents")
    Ensure-Dir -Path (Join-Path $NoaRoot "workspace\tools")

    # P2P server directories
    Write-Host "  Creating P2P structure..." -ForegroundColor Gray
    Ensure-Dir -Path (Join-Path $NoaRoot "p2p\nodes")
    Ensure-Dir -Path (Join-Path $NoaRoot "p2p\storage")
    Ensure-Dir -Path (Join-Path $NoaRoot "p2p\compute")
    Ensure-Dir -Path (Join-Path $NoaRoot "p2p\network")

    # Multi-provider AI system structure
    Write-Host "  Creating AI structure..." -ForegroundColor Gray
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\providers")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\shared")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\devices")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\orchestration")

    Ensure-Dir -Path (Join-Path $NoaRoot "ai\shared\agents")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\shared\workflows")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\shared\prompts")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\shared\skills")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\shared\tools")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\shared\models")

    Ensure-Dir -Path (Join-Path $NoaRoot "ai\providers\local")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\providers\cloud")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\providers\hybrid")

    Ensure-Dir -Path (Join-Path $NoaRoot "ai\devices\registered")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\devices\active")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\devices\capabilities")

    Ensure-Dir -Path (Join-Path $NoaRoot "ai\orchestration\tasks")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\orchestration\queues")
    Ensure-Dir -Path (Join-Path $NoaRoot "ai\orchestration\scheduling")

    # Local Git CI/CD system structure
    Write-Host "  Creating Git CI/CD structure..." -ForegroundColor Gray
    Ensure-Dir -Path (Join-Path $NoaRoot "git\repos")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\prs")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\merges")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\conflicts")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\ci-cd")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\mirrors")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\hooks")

    Ensure-Dir -Path (Join-Path $NoaRoot "git\repos\bare")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\repos\working")

    Ensure-Dir -Path (Join-Path $NoaRoot "git\prs\open")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\prs\merged")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\prs\closed")

    Ensure-Dir -Path (Join-Path $NoaRoot "git\conflicts\pending")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\conflicts\resolved")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\conflicts\ai-resolved")

    Ensure-Dir -Path (Join-Path $NoaRoot "git\ci-cd\pipelines")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\ci-cd\jobs")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\ci-cd\artifacts")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\ci-cd\logs")

    Ensure-Dir -Path (Join-Path $NoaRoot "git\mirrors\github")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\mirrors\local")

    Ensure-Dir -Path (Join-Path $NoaRoot "git\hooks\pre-commit")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\hooks\post-commit")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\hooks\pre-push")
    Ensure-Dir -Path (Join-Path $NoaRoot "git\hooks\post-merge")

    Write-Host "Directory structure created successfully." -ForegroundColor Green
} else {
    Write-Host "Skipping directory structure creation (already exists or -SkipStructure specified)." -ForegroundColor Gray
}

### 3. Create .noa marker file ###############################################

Write-Host "Creating .noa marker file..." -ForegroundColor Cyan

# Create .noa marker file content
$noaMarkerContent = @'
# NOA Configuration
# This directory is the root of the NOA monorepo
# NOA is a P2P server for connecting user devices for dynamically shared compute and storage
# All paths must be relative to {0}
# AI-first, conversational UI/OS - minimal extensions, CLI tools preferred
'@ -f $NoaRoot

$noaMarkerContent | Out-File -FilePath (Join-Path $NoaRoot ".noa") -Encoding UTF8 -Force
Write-Host "  Created .noa marker file" -ForegroundColor Green

### 4. Create Windows PowerShell profile for NOA environment ##################

Write-Host "Creating NOA PowerShell profile..." -ForegroundColor Cyan

$profilePath = Join-Path $NoaRoot "noa-profile.ps1"

# Write the profile content line by line to avoid any parsing issues
$sb = [System.Text.StringBuilder]::new()
$null = $sb.AppendLine('# NOA Environment Configuration for Windows PowerShell')
$null = $sb.AppendLine('# Ensures all paths stay within NOA_ROOT')
$null = $sb.AppendLine('# NOA: P2P server for connecting user devices for dynamically shared compute and storage')
$null = $sb.AppendLine('')
$null = $sb.AppendLine("`$env:NOA_ROOT = `"$NoaRoot`"")
$null = $sb.AppendLine('$env:NOA_REPOS = "$env:NOA_ROOT\repos"')
$null = $sb.AppendLine('$env:NOA_CONTAINERS = "$env:NOA_ROOT\containers"')
$null = $sb.AppendLine('$env:NOA_WORKSPACE = "$env:NOA_ROOT\workspace"')
$null = $sb.AppendLine('$env:NOA_CONFIG = "$env:NOA_ROOT\config"')
$null = $sb.AppendLine('$env:NOA_SCRIPTS = "$env:NOA_ROOT\scripts"')
$null = $sb.AppendLine('$env:NOA_LOGS = "$env:NOA_ROOT\logs"')
$null = $sb.AppendLine('$env:NOA_TMP = "$env:NOA_ROOT\tmp"')
$null = $sb.AppendLine('$env:NOA_P2P = "$env:NOA_ROOT\p2p"')
$null = $sb.AppendLine('$env:NOA_AI = "$env:NOA_ROOT\ai"')
$null = $sb.AppendLine('$env:NOA_AI_SHARED = "$env:NOA_AI\shared"')
$null = $sb.AppendLine('$env:NOA_AI_PROVIDERS = "$env:NOA_AI\providers"')
$null = $sb.AppendLine('$env:NOA_AI_DEVICES = "$env:NOA_AI\devices"')
$null = $sb.AppendLine('$env:NOA_AI_ORCHESTRATION = "$env:NOA_AI\orchestration"')
$null = $sb.AppendLine('$env:NOA_GIT = "$env:NOA_ROOT\git"')
$null = $sb.AppendLine('$env:NOA_GIT_REPOS = "$env:NOA_GIT\repos"')
$null = $sb.AppendLine('$env:NOA_GIT_PRS = "$env:NOA_GIT\prs"')
$null = $sb.AppendLine('$env:NOA_GIT_CONFLICTS = "$env:NOA_GIT\conflicts"')
$null = $sb.AppendLine('$env:NOA_GIT_CI_CD = "$env:NOA_GIT\ci-cd"')
$null = $sb.AppendLine('$env:NOA_GIT_MIRRORS = "$env:NOA_GIT\mirrors"')
$null = $sb.AppendLine('$env:NOA_BIN = "$env:NOA_ROOT\bin"')
$null = $sb.AppendLine('$env:NOA_ETC = "$env:NOA_ROOT\etc"')
$null = $sb.AppendLine('$env:NOA_LIB = "$env:NOA_ROOT\lib"')
$null = $sb.AppendLine('$env:NOA_OPT = "$env:NOA_ROOT\opt"')
$null = $sb.AppendLine('$env:NOA_SYS = "$env:NOA_ROOT\sys"')
$null = $sb.AppendLine('$env:NOA_INIT = "$env:NOA_ROOT\init"')
$null = $sb.AppendLine('')
$null = $sb.AppendLine('# Add NOA directories to PATH')
$null = $sb.AppendLine('function Add-NoaPath {')
$null = $sb.AppendLine('    param([string]$dir)')
$null = $sb.AppendLine('    if (Test-Path $dir) {')
$null = $sb.AppendLine('        if ($env:PATH -notlike "*$dir*") {')
$null = $sb.AppendLine('            $env:PATH = ' + [char]34 + '$dir;$env:PATH' + [char]34)
$null = $sb.AppendLine('        }')
$null = $sb.AppendLine('    }')
$null = $sb.AppendLine('}')
$null = $sb.AppendLine('')
$null = $sb.AppendLine('# Add NOA bin and scripts to PATH (highest priority)')
$null = $sb.AppendLine('Add-NoaPath $env:NOA_BIN')
$null = $sb.AppendLine('Add-NoaPath $env:NOA_SCRIPTS')
$null = $sb.AppendLine('')
$null = $sb.AppendLine('# Function to validate paths are within NOA_ROOT')
$null = $sb.AppendLine('function Test-NoaPath {')
$null = $sb.AppendLine('    param([string]$path)')
$null = $sb.AppendLine('    $noaRootPath = $env:NOA_ROOT')
$null = $sb.AppendLine('    # Normalize paths for comparison (remove trailing backslashes)')
$null = $sb.AppendLine('    $normalizedPath = $path.TrimEnd([char]92)')
$null = $sb.AppendLine('    $normalizedRoot = $noaRootPath.TrimEnd([char]92)')
$null = $sb.AppendLine('    # Use StartsWith for explicit path validation instead of pattern matching')
$null = $sb.AppendLine('    if (-not $normalizedPath.StartsWith($normalizedRoot, [System.StringComparison]::OrdinalIgnoreCase)) {')
$null = $sb.AppendLine('        Write-Error ' + [char]34 + 'Path must be within NOA_ROOT - Path was $path' + [char]34)
$null = $sb.AppendLine('        return $false')
$null = $sb.AppendLine('    }')
$null = $sb.AppendLine('    return $true')
$null = $sb.AppendLine('}')
$null = $sb.AppendLine('')
$null = $sb.AppendLine('# Aliases for quick navigation')
$null = $sb.AppendLine('function cda { Set-Location $env:NOA_ROOT }')
$null = $sb.AppendLine('function cdr { Set-Location $env:NOA_REPOS }')
$null = $sb.AppendLine('function cdc { Set-Location $env:NOA_CONTAINERS }')
$null = $sb.AppendLine('function cdw { Set-Location $env:NOA_WORKSPACE }')
$null = $sb.AppendLine('function cdp { Set-Location $env:NOA_P2P }')
$null = $sb.AppendLine('function cdai { Set-Location $env:NOA_AI }')
$null = $sb.AppendLine('function cdgit { Set-Location $env:NOA_GIT }')
$null = $sb.AppendLine('')
$null = $sb.AppendLine('Write-Host ' + [char]34 + 'NOA environment loaded. Root: $env:NOA_ROOT' + [char]34 + ' -ForegroundColor Green')

[System.IO.File]::WriteAllText($profilePath, $sb.ToString(), [System.Text.Encoding]::UTF8)
Write-Host "  Created NOA PowerShell profile: $profilePath" -ForegroundColor Green
Write-Host "  To use: . '$profilePath'" -ForegroundColor Yellow

### 5. Create initial configuration files #####################################

Write-Host "Creating initial configuration files..." -ForegroundColor Cyan

# Create placeholder config files
$configFiles = @{
    "config\noa-server.json" = @'
{
  "name": "NOA Server",
  "version": "1.0.0",
  "port": 8080,
  "p2p": {
    "enabled": true,
    "port": 8080
  }
}
'@
    "config\ai-providers.json" = @'
{
  "providers": {
    "local": {
      "enabled": true,
      "priority": 1
    },
    "cloud": {
      "enabled": false,
      "priority": 2
    },
    "hybrid": {
      "enabled": false,
      "priority": 3
    }
  }
}
'@
    "config\device-orchestration.json" = @'
{
  "devices": [],
  "capabilities": {}
}
'@
    "config\git-local-cicd.json" = @'
{
  "pipelines": [],
  "enabled": true
}
'@
    "config\git-conflict-ai.json" = @'
{
  "enabled": true,
  "strategy": "ai-mlops"
}
'@
    "config\git-pr-workflow.json" = @'
{
  "enabled": true,
  "autoResolve": false
}
'@
}

foreach ($file in $configFiles.GetEnumerator()) {
    $filePath = Join-Path $NoaRoot $file.Key
    $file.Value | Out-File -FilePath $filePath -Encoding UTF8 -Force
    Write-Host "  Created $($file.Key)" -ForegroundColor Gray
}

### 6. Final verification ####################################################

Write-Host ""
Write-Host "=== Setup Complete ===" -ForegroundColor Green
Write-Host ""
Write-Host "NOA Structure Summary:" -ForegroundColor Cyan
Write-Host "  Root Directory: $NoaRoot" -ForegroundColor White
Write-Host "  Repos: $NoaRoot\repos\[github|local|external]" -ForegroundColor White
Write-Host "  Containers: $NoaRoot\containers\[docker|compose|volumes]" -ForegroundColor White
Write-Host "  Workspace: $NoaRoot\workspace\[projects|agents|tools]" -ForegroundColor White
Write-Host "  P2P: $NoaRoot\p2p\[nodes|storage|compute|network]" -ForegroundColor White
Write-Host "  AI: $NoaRoot\ai\[providers|shared|devices|orchestration]" -ForegroundColor White
Write-Host "  Git: $NoaRoot\git\[repos|prs|conflicts|ci-cd|mirrors|hooks]" -ForegroundColor White
Write-Host ""
Write-Host "To activate NOA environment in PowerShell:" -ForegroundColor Yellow
Write-Host "  . '$profilePath'" -ForegroundColor White
Write-Host ""
Write-Host "Quick navigation aliases (after loading profile):" -ForegroundColor Yellow
Write-Host "  cda  # cd to NOA root" -ForegroundColor White
Write-Host "  cdr  # cd to repos" -ForegroundColor White
Write-Host "  cdc  # cd to containers" -ForegroundColor White
Write-Host "  cdw  # cd to workspace" -ForegroundColor White
Write-Host "  cdp  # cd to p2p" -ForegroundColor White
Write-Host "  cdai # cd to ai" -ForegroundColor White
Write-Host "  cdgit # cd to git" -ForegroundColor White
Write-Host ""
