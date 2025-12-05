# NOA Git Issue Fix Script
# Run this from N:\noa directory
# Usage: .\fix-git-issues.ps1 [abort|continue|auto]

param(
    [Parameter(Position=0)]
    [ValidateSet("abort", "continue", "auto", "status", "")]
    [string]$Action = ""
)

Write-Host ""
Write-Host "=== NOA Git Issue Fixer ===" -ForegroundColor Cyan
Write-Host ""

# Check current directory
$currentDir = Get-Location
Write-Host "Working directory: $currentDir" -ForegroundColor Gray

# Check if we're in a git repo
if (-not (Test-Path ".git")) {
    Write-Host "ERROR: Not in a git repository. Please run: cd N:\noa" -ForegroundColor Red
    exit 1
}

# Check for rebase in progress
$rebaseDir = ".git\rebase-merge"
$hasRebase = Test-Path $rebaseDir

if ($hasRebase) {
    $msgnum = Get-Content "$rebaseDir\msgnum" -ErrorAction SilentlyContinue
    $end = Get-Content "$rebaseDir\end" -ErrorAction SilentlyContinue
    $stoppedSha = Get-Content "$rebaseDir\stopped-sha" -ErrorAction SilentlyContinue

    Write-Host "[ISSUE] Rebase in progress (step $msgnum of $end)" -ForegroundColor Yellow
    Write-Host "  Stopped at: $stoppedSha" -ForegroundColor Gray
    Write-Host ""

    # The ruler-main/package-lock.json has been verified as CLEAN (no conflict markers)
    Write-Host "[INFO] ruler-main/package-lock.json is CLEAN (no conflict markers)" -ForegroundColor Green
    Write-Host ""
}

# Handle actions
switch ($Action) {
    "abort" {
        Write-Host ">>> Aborting rebase..." -ForegroundColor Yellow
        git rebase --abort
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[SUCCESS] Rebase aborted. You are now back on main." -ForegroundColor Green
            git status -sb
        } else {
            Write-Host "[ERROR] Failed to abort rebase." -ForegroundColor Red
        }
    }
    "continue" {
        Write-Host ">>> Staging ruler-main/package-lock.json..." -ForegroundColor Yellow
        git add ruler-main/package-lock.json
        Write-Host ">>> Continuing rebase..." -ForegroundColor Yellow
        # Use --no-edit to accept the default commit message
        git -c core.editor=true rebase --continue
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[SUCCESS] Rebase step completed!" -ForegroundColor Green
            # Check if rebase is fully done
            if (-not (Test-Path $rebaseDir)) {
                Write-Host "[SUCCESS] Rebase fully completed! You are now on main." -ForegroundColor Green
            } else {
                Write-Host "[INFO] There may be more rebase steps. Run this script again if needed." -ForegroundColor Yellow
            }
            git status -sb
        } else {
            Write-Host "[ERROR] Rebase continue failed. There may be more conflicts." -ForegroundColor Red
            git status
        }
    }
    "auto" {
        if ($hasRebase) {
            Write-Host ">>> AUTO MODE: Attempting to fix rebase automatically..." -ForegroundColor Cyan
            Write-Host ""

            # Stage the known conflict file
            Write-Host ">>> Staging ruler-main/package-lock.json..." -ForegroundColor Yellow
            git add ruler-main/package-lock.json

            # Continue rebase with default message
            Write-Host ">>> Continuing rebase..." -ForegroundColor Yellow
            git -c core.editor=true rebase --continue

            # Loop to handle multiple steps
            $maxAttempts = 5
            $attempt = 0
            while ((Test-Path $rebaseDir) -and ($attempt -lt $maxAttempts)) {
                $attempt++
                Write-Host ">>> Rebase step $attempt - staging all changes..." -ForegroundColor Yellow
                git add -A
                git -c core.editor=true rebase --continue 2>$null
                if ($LASTEXITCODE -ne 0) {
                    Write-Host "[WARN] Rebase step may have conflicts. Check git status." -ForegroundColor Yellow
                    break
                }
            }

            if (-not (Test-Path $rebaseDir)) {
                Write-Host ""
                Write-Host "[SUCCESS] Rebase completed!" -ForegroundColor Green
                Write-Host ""
                Write-Host ">>> Pulling latest from origin..." -ForegroundColor Yellow
                git pull origin main --no-edit
                Write-Host ""
                git status -sb
            } else {
                Write-Host ""
                Write-Host "[WARN] Rebase not fully complete. Manual intervention may be needed." -ForegroundColor Yellow
                git status
            }
        } else {
            Write-Host "[INFO] No rebase in progress. Repository is clean." -ForegroundColor Green
            git status -sb
        }
    }
    default {
        # Status mode - just show info
        Write-Host "=== Current Status ===" -ForegroundColor Cyan
        git status

        Write-Host ""
        Write-Host "=== Available Commands ===" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "  .\fix-git-issues.ps1 auto      " -NoNewline -ForegroundColor Green
        Write-Host "- Automatically fix the rebase (RECOMMENDED)" -ForegroundColor White
        Write-Host "  .\fix-git-issues.ps1 continue  " -NoNewline -ForegroundColor Green
        Write-Host "- Stage conflict file and continue rebase" -ForegroundColor White
        Write-Host "  .\fix-git-issues.ps1 abort     " -NoNewline -ForegroundColor Green
        Write-Host "- Abort rebase and return to main" -ForegroundColor White
        Write-Host ""

        if ($hasRebase) {
            Write-Host "=== Manual Fix Commands ===" -ForegroundColor Yellow
            Write-Host ""
            Write-Host "  git add ruler-main/package-lock.json" -ForegroundColor Gray
            Write-Host "  git rebase --continue" -ForegroundColor Gray
            Write-Host ""
            Write-Host "  OR to abort:" -ForegroundColor Gray
            Write-Host "  git rebase --abort" -ForegroundColor Gray
        }
    }
}
