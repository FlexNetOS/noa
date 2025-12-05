<#
.SYNOPSIS
    Quick sync: pull, commit, push, and optionally merge to main.

.DESCRIPTION
    One-command workflow for single-user Git automation.

.PARAMETER Message
    Commit message (optional, auto-generated if not provided)

.EXAMPLE
    .\sync.ps1
    .\sync.ps1 "feat: add new feature"
#>

param(
    [Parameter(Position=0)]
    [string]$Message = ""
)

$scriptPath = Join-Path $PSScriptRoot "git-push.ps1"
& $scriptPath -Message $Message -AutoMerge
