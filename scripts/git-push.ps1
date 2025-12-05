<#
.SYNOPSIS
    Push changes to develop and auto-create PR to main.

.DESCRIPTION
    This script commits all changes, pushes to develop, and creates a PR to main.
    Designed for single-user automated workflow.

.PARAMETER Message
    Commit message. If not provided, generates one based on changed files.

.PARAMETER AutoMerge
    Automatically merge the PR after creation.

.EXAMPLE
    .\git-push.ps1 -Message "feat: add new feature"
    .\git-push.ps1 -AutoMerge
#>

param(
    [string]$Message = "",
    [switch]$AutoMerge
)

$ErrorActionPreference = "Stop"

# Ensure we're on develop
$currentBranch = git branch --show-current
if ($currentBranch -ne "develop") {
    Write-Host "Switching to develop branch..." -ForegroundColor Yellow
    git checkout develop
}

# Pull latest
Write-Host "Pulling latest changes..." -ForegroundColor Cyan
git pull origin develop

# Check for changes
$status = git status --porcelain
if (-not $status) {
    Write-Host "No changes to commit." -ForegroundColor Green
    exit 0
}

# Generate commit message if not provided
if (-not $Message) {
    $changedFiles = ($status | ForEach-Object { $_.Substring(3) } | Select-Object -First 3) -join ", "
    $Message = "chore: update $changedFiles"
    if ($status.Count -gt 3) {
        $Message += " and $($status.Count - 3) more"
    }
}

# Stage and commit
Write-Host "Staging changes..." -ForegroundColor Cyan
git add -A

Write-Host "Committing: $Message" -ForegroundColor Cyan
git commit -m $Message

# Push to develop
Write-Host "Pushing to develop..." -ForegroundColor Cyan
git push origin develop

# Check if gh is available
$ghAvailable = Get-Command gh -ErrorAction SilentlyContinue
if (-not $ghAvailable) {
    Write-Host "GitHub CLI not found. Install with: winget install GitHub.cli" -ForegroundColor Yellow
    exit 0
}

# Create or update PR
Write-Host "Checking for existing PR..." -ForegroundColor Cyan
$existingPR = gh pr list --base main --head develop --json number --jq '.[0].number' 2>$null

if ($existingPR) {
    Write-Host "PR #$existingPR already exists" -ForegroundColor Green
} else {
    Write-Host "Creating PR to main..." -ForegroundColor Cyan
    gh pr create --base main --head develop --title $Message --body "Automated PR from develop to main."
    $existingPR = gh pr list --base main --head develop --json number --jq '.[0].number'
}

# Auto-merge if requested
if ($AutoMerge -and $existingPR) {
    Write-Host "Merging PR #$existingPR..." -ForegroundColor Cyan
    gh pr merge $existingPR --squash --delete-branch=false
    Write-Host "PR merged successfully!" -ForegroundColor Green
}

Write-Host "`nDone!" -ForegroundColor Green
