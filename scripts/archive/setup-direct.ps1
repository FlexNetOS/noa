# Direct setup script with logging
$logFile = "N:\noa\setup-log.txt"
$NoaRoot = "N:\noa"

function Log {
    param([string]$Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$timestamp - $Message" | Out-File -FilePath $logFile -Append -Encoding UTF8
    Write-Host $Message
}

Log "=== Starting NOA Setup ==="
Log "NOA Root: $NoaRoot"

# Test write access
try {
    "test" | Out-File -FilePath "$NoaRoot\test-write.txt" -Encoding UTF8
    if (Test-Path "$NoaRoot\test-write.txt") {
        Remove-Item "$NoaRoot\test-write.txt" -Force
        Log "[OK] Write access confirmed"
    } else {
        Log "[FAIL] Write access FAILED"
        exit 1
    }
} catch {
    Log "[FAIL] Write test failed: $($_.Exception.Message)"
    exit 1
}

# Create directories using .NET
$mainDirs = @("repos", "config", "scripts", "logs", "tmp", "p2p", "ai", "git", "bin", "etc", "lib", "opt", "sys", "init", "containers", "workspace")

Log "Creating main directories..."
foreach ($dir in $mainDirs) {
    $fullPath = Join-Path $NoaRoot $dir
    try {
        if (-not [System.IO.Directory]::Exists($fullPath)) {
            [System.IO.Directory]::CreateDirectory($fullPath) | Out-Null
            if ([System.IO.Directory]::Exists($fullPath)) {
                Log "  [OK] Created: $dir"
            } else {
                Log "  [FAIL] Failed: $dir"
            }
        } else {
            Log "  [EXISTS] Already exists: $dir"
        }
    } catch {
        Log "  [ERROR] Error: $dir - $($_.Exception.Message)"
    }
}

# Verify
Log ""
Log "=== Verification ==="
$created = 0
foreach ($dir in $mainDirs) {
    $fullPath = Join-Path $NoaRoot $dir
    if ([System.IO.Directory]::Exists($fullPath)) {
        $created++
        Log "  [OK] $dir exists"
    } else {
        Log "  [MISSING] $dir MISSING"
    }
}

Log ""
Log "Created $created of $($mainDirs.Count) directories"
Log "=== Setup Complete ==="
