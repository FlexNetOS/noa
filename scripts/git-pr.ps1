<#
.SYNOPSIS
    NOA Git PR Workflow Tool for Windows

.DESCRIPTION
    Windows equivalent of scripts/git-pr (bash)
    Manages pull request workflows

.PARAMETER Action
    Action: create, list, review, merge, status

.PARAMETER Title
    PR title (for create)

.PARAMETER Branch
    Branch name

.PARAMETER NoaRoot
    NOA root directory

.EXAMPLE
    .\git-pr.ps1 -Action list
    .\git-pr.ps1 -Action create -Title "Add feature X" -Branch "feature/x"
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("create", "list", "review", "merge", "status")]
    [string]$Action,

    [string]$Title,
    [string]$Branch,
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$PR_configs = Join-Path $NoaRoot "configs/git-pr-workflow.json"

# Check for gh CLI
$ghAvailable = Get-Command "gh" -ErrorAction SilentlyContinue

switch ($Action) {
    "create" {
        Write-Host "Creating Pull Request..." -ForegroundColor Cyan

        if (-not $Title) {
            Write-Error "Usage: git-pr.ps1 -Action create -Title <title> [-Branch <branch>]"
        }

        if ($ghAvailable) {
            $args = @("pr", "create", "--title", $Title)
            if ($Branch) { $args += "--base", $Branch }
            & gh @args
        } else {
            Write-Host "  GitHub CLI (gh) not available" -ForegroundColor Yellow
            Write-Host "  Install with: winget install GitHub.cli" -ForegroundColor Gray
        }
    }

    "list" {
        Write-Host "Open Pull Requests:" -ForegroundColor Cyan

        if ($ghAvailable) {
            gh pr list
        } else {
            Write-Host "  GitHub CLI (gh) not available" -ForegroundColor Yellow
        }
    }

    "review" {
        Write-Host "PR Review:" -ForegroundColor Cyan

        if ($ghAvailable) {
            gh pr view
        } else {
            Write-Host "  GitHub CLI (gh) not available" -ForegroundColor Yellow
        }
    }

    "merge" {
        Write-Host "Merging PR..." -ForegroundColor Cyan

        if ($ghAvailable) {
            gh pr merge --auto --squash
        } else {
            Write-Host "  GitHub CLI (gh) not available" -ForegroundColor Yellow
        }
    }

    "status" {
        Write-Host "PR Status:" -ForegroundColor Cyan

        if ($ghAvailable) {
            gh pr status
        } else {
            Write-Host "  GitHub CLI (gh) not available" -ForegroundColor Yellow
        }
    }
}

