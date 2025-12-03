# Script to check and fix worktree locations
Write-Host "=== Checking Git Worktrees ===" -ForegroundColor Cyan

# List all worktrees
Write-Host "`nCurrent worktrees:" -ForegroundColor Yellow
git worktree list

# Check for worktrees on C drive
Write-Host "`n=== Checking for worktrees on C: drive ===" -ForegroundColor Cyan
$worktrees = git worktree list --porcelain 2>&1
if ($worktrees) {
    $worktrees | ForEach-Object {
        if ($_ -match '^worktree (.+)$') {
            $path = $matches[1]
            if ($path -like 'C:*') {
                Write-Host "FOUND worktree on C: drive: $path" -ForegroundColor Red
            }
        }
    }
} else {
    Write-Host "No worktrees found or error reading worktrees" -ForegroundColor Yellow
}

# Check git status
Write-Host "`n=== Git Status ===" -ForegroundColor Cyan
git status --short

# Check for uncommitted changes
$status = git status --porcelain
if ($status) {
    Write-Host "`nUncommitted changes found:" -ForegroundColor Yellow
    git status --short
} else {
    Write-Host "`nWorking directory is clean" -ForegroundColor Green
}
