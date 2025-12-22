# Phase 0 Quality Verification - Reproduction Guide

This document provides exact steps to reproduce the Phase 0 quality verification results.

---

## Environment Requirements

### Windows (PowerShell)
- **OS**: Windows 10/11 or Windows Server 2019+
- **PowerShell**: PowerShell 7.0+ (pwsh)
- **Tools**:
  - Git (for cloning repository)
  - Internet connection (for downloading tools)

### Linux (Bash)
- **OS**: Ubuntu 20.04+, Debian 11+, or equivalent
- **Shell**: Bash 4.4+
- **Tools**:
  - `curl` or `wget` (for downloads)
  - `jq` (for JSON parsing, optional)
  - `sha256sum` (for checksum verification)
  - Git (for cloning repository)

### macOS (Bash)
- **OS**: macOS 11+ (Big Sur or later)
- **Shell**: Bash 4.4+ (via Homebrew if needed)
- **Tools**: Same as Linux

---

## Step 1: Clone Repository

```bash
# Clone the NOA repository
git clone <repository-url> noa
cd noa
```

---

## Step 2: Verify Script Structure

### Windows (PowerShell)
```powershell
# Count scripts
$ps1Count = (Get-ChildItem -Path "scripts/bootstrap" -Recurse -Filter "*.ps1" | Measure-Object).Count
$shCount = (Get-ChildItem -Path "scripts/bootstrap" -Recurse -Filter "*.sh" | Measure-Object).Count
Write-Host "PowerShell scripts: $ps1Count"
Write-Host "Bash scripts: $shCount"

# Verify core scripts exist
$coreScripts = @("bootstrap.ps1", "bootstrap.sh")
foreach ($script in $coreScripts) {
    $path = "scripts/bootstrap/$script"
    if (Test-Path $path) {
        Write-Host "[OK] $path exists"
    } else {
        Write-Host "[FAIL] $path missing"
    }
}
```

### Linux/macOS (Bash)
```bash
# Count scripts
ps1_count=$(find scripts/bootstrap -name "*.ps1" | wc -l)
sh_count=$(find scripts/bootstrap -name "*.sh" | wc -l)
echo "PowerShell scripts: $ps1_count"
echo "Bash scripts: $sh_count"

# Verify core scripts exist
for script in bootstrap.ps1 bootstrap.sh; do
    path="scripts/bootstrap/$script"
    if [[ -f "$path" ]]; then
        echo "[OK] $path exists"
    else
        echo "[FAIL] $path missing"
    fi
done
```

**Expected Output**: All core scripts should exist.

---

## Step 3: Verify Error Handling

### Windows (PowerShell)
```powershell
# Check bootstrap.ps1
$content = Get-Content "scripts/bootstrap/bootstrap.ps1" -Raw
if ($content -match '\$ErrorActionPreference\s*=\s*"Stop"') {
    Write-Host "[OK] bootstrap.ps1 has ErrorActionPreference = Stop"
} else {
    Write-Host "[FAIL] bootstrap.ps1 missing ErrorActionPreference = Stop"
}

# Check library scripts
$libScripts = Get-ChildItem "scripts/bootstrap/lib/*.ps1"
foreach ($script in $libScripts) {
    $content = Get-Content $script.FullName -Raw
    if ($content -match '\$ErrorActionPreference\s*=\s*"Stop"') {
        Write-Host "[OK] $($script.Name) has ErrorActionPreference = Stop"
    } else {
        Write-Host "[WARN] $($script.Name) missing ErrorActionPreference = Stop"
    }
}
```

### Linux/macOS (Bash)
```bash
# Check bootstrap.sh
if grep -q "set -euo pipefail" "scripts/bootstrap/bootstrap.sh"; then
    echo "[OK] bootstrap.sh has set -euo pipefail"
else
    echo "[FAIL] bootstrap.sh missing set -euo pipefail"
fi

# Check library scripts
for script in scripts/bootstrap/lib/*.sh; do
    if grep -q "set -euo pipefail" "$script"; then
        echo "[OK] $(basename "$script") has set -euo pipefail"
    else
        echo "[WARN] $(basename "$script") missing set -euo pipefail"
    fi
done
```

**Expected Output**: All scripts should have error handling.

---

## Step 4: Verify Cross-Platform Parity

### Windows (PowerShell)
```powershell
# Run cross-platform parity check
& "scripts/bootstrap/verify/cross-platform-parity.ps1"
```

### Linux/macOS (Bash)
```bash
# Run cross-platform parity check
bash scripts/bootstrap/verify/cross-platform-parity.sh
```

**Expected Output**: All core scripts should have matching PS1/.sh pairs.

---

## Step 5: Verify Checksum Support

### Windows (PowerShell)
```powershell
# Check download.ps1
$content = Get-Content "scripts/bootstrap/lib/download.ps1" -Raw
if ($content -match 'Checksum|checksum') {
    Write-Host "[OK] download.ps1 supports checksum verification"
} else {
    Write-Host "[FAIL] download.ps1 missing checksum support"
}

# Check if Checksum parameter exists
$func = Get-Content "scripts/bootstrap/lib/download.ps1" | Select-String -Pattern "function Get-NoaDownload" -Context 0,50
if ($func -match 'Checksum') {
    Write-Host "[OK] Get-NoaDownload accepts Checksum parameter"
} else {
    Write-Host "[FAIL] Get-NoaDownload missing Checksum parameter"
}
```

### Linux/macOS (Bash)
```bash
# Check download.sh
if grep -q "checksum" "scripts/bootstrap/lib/download.sh"; then
    echo "[OK] download.sh supports checksum verification"
else
    echo "[FAIL] download.sh missing checksum support"
fi

# Check if --checksum option exists
if grep -q "--checksum" "scripts/bootstrap/lib/download.sh"; then
    echo "[OK] noa_download accepts --checksum option"
else
    echo "[FAIL] noa_download missing --checksum option"
fi
```

**Expected Output**: Both download functions should support checksums.

---

## Step 6: Verify Logging

### Windows (PowerShell)
```powershell
# Check logging.ps1 exists and has key functions
$loggingPath = "scripts/bootstrap/lib/logging.ps1"
if (Test-Path $loggingPath) {
    $content = Get-Content $loggingPath -Raw
    $functions = @("Initialize-Logging", "Write-Log")
    foreach ($func in $functions) {
        if ($content -match "function $func") {
            Write-Host "[OK] $func exists in logging.ps1"
        } else {
            Write-Host "[FAIL] $func missing in logging.ps1"
        }
    }
} else {
    Write-Host "[FAIL] logging.ps1 not found"
}
```

### Linux/macOS (Bash)
```bash
# Check logging.sh exists and has key functions
if [[ -f "scripts/bootstrap/lib/logging.sh" ]]; then
    for func in initialize_logging log; do
        if grep -q "^$func()" "scripts/bootstrap/lib/logging.sh"; then
            echo "[OK] $func exists in logging.sh"
        else
            echo "[FAIL] $func missing in logging.sh"
        fi
    done
else
    echo "[FAIL] logging.sh not found"
fi
```

**Expected Output**: All logging functions should exist.

---

## Step 7: Verify State Management

### Windows (PowerShell)
```powershell
# Check state.ps1 exists and has key functions
$statePath = "scripts/bootstrap/lib/state.ps1"
if (Test-Path $statePath) {
    $content = Get-Content $statePath -Raw
    $functions = @("Initialize-BootstrapState", "Get-ToolState", "Set-ToolState")
    foreach ($func in $functions) {
        if ($content -match "function $func") {
            Write-Host "[OK] $func exists in state.ps1"
        } else {
            Write-Host "[FAIL] $func missing in state.ps1"
        }
    }
} else {
    Write-Host "[FAIL] state.ps1 not found"
}
```

### Linux/macOS (Bash)
```bash
# Check state.sh exists and has key functions
if [[ -f "scripts/bootstrap/lib/state.sh" ]]; then
    for func in initialize_bootstrap_state get_tool_state set_tool_state; do
        if grep -q "^$func()" "scripts/bootstrap/lib/state.sh"; then
            echo "[OK] $func exists in state.sh"
        else
            echo "[FAIL] $func missing in state.sh"
        fi
    done
else
    echo "[FAIL] state.sh not found"
fi
```

**Expected Output**: All state management functions should exist.

---

## Step 8: Run Test Suite

### Windows (PowerShell)
```powershell
# Run library tests
& "scripts/bootstrap/tests/test-libraries.ps1"
```

### Linux/macOS (Bash)
```bash
# Run library tests
bash scripts/bootstrap/tests/test-libraries.sh
```

**Expected Output**: All tests should pass.

---

## Step 9: Generate SHA-256 Hashes

### Windows (PowerShell)
```powershell
# Generate hashes for all bootstrap scripts
$files = Get-ChildItem -Path "scripts/bootstrap" -Recurse -File -Include "*.ps1","*.sh" | Where-Object { $_.FullName -notlike "*test*" }
$hashes = @()
foreach ($f in $files) {
    $hash = (Get-FileHash $f.FullName -Algorithm SHA256).Hash
    $relPath = $f.FullName.Replace((Get-Location).Path + "\", "").Replace("\", "/")
    $hashes += "$hash  $relPath"
}
$hashes | Out-File -FilePath "specs/001-noa-seed-foundation/checklists/phase0-hashes.txt" -Encoding utf8
Write-Host "Generated $($hashes.Count) hashes"
```

### Linux/macOS (Bash)
```bash
# Generate hashes for all bootstrap scripts
find scripts/bootstrap -type f \( -name "*.ps1" -o -name "*.sh" \) ! -path "*test*" | while read -r file; do
    hash=$(sha256sum "$file" | cut -d' ' -f1)
    rel_path=$(echo "$file" | sed "s|^$(pwd)/||")
    echo "$hash  $rel_path"
done > specs/001-noa-seed-foundation/checklists/phase0-hashes.txt
echo "Generated $(wc -l < specs/001-noa-seed-foundation/checklists/phase0-hashes.txt) hashes"
```

**Expected Output**: Hash file should be created with all script hashes.

---

## Step 10: Verify Quality Report

### Windows (PowerShell)
```powershell
# Check quality report exists
$reportPath = "specs/001-noa-seed-foundation/checklists/phase0-quality-report.md"
if (Test-Path $reportPath) {
    Write-Host "[OK] Quality report exists"

    # Count check items
    $content = Get-Content $reportPath -Raw
    $passCount = ([regex]::Matches($content, '\[X\]').Count)
    $failCount = ([regex]::Matches($content, '\[ \]').Count)
    Write-Host "Passed checks: $passCount"
    Write-Host "Pending checks: $failCount"
} else {
    Write-Host "[FAIL] Quality report not found"
}
```

### Linux/macOS (Bash)
```bash
# Check quality report exists
if [[ -f "specs/001-noa-seed-foundation/checklists/phase0-quality-report.md" ]]; then
    echo "[OK] Quality report exists"

    # Count check items
    pass_count=$(grep -c "\[X\]" "specs/001-noa-seed-foundation/checklists/phase0-quality-report.md" || echo "0")
    fail_count=$(grep -c "\[ \]" "specs/001-noa-seed-foundation/checklists/phase0-quality-report.md" || echo "0")
    echo "Passed checks: $pass_count"
    echo "Pending checks: $fail_count"
else
    echo "[FAIL] Quality report not found"
fi
```

**Expected Output**: Quality report should exist with check items.

---

## Troubleshooting

### Issue: Scripts not found
**Solution**: Ensure you're in the repository root directory (`noa/`).

### Issue: PowerShell version too old
**Solution**: Install PowerShell 7.0+ from https://aka.ms/powershell-release

### Issue: Bash version too old
**Solution**: Update Bash or use Homebrew to install newer version.

### Issue: Tests fail
**Solution**: Check that all library scripts are present and have correct syntax.

### Issue: Hash generation fails
**Solution**: Ensure you have write permissions to `specs/001-noa-seed-foundation/checklists/`.

---

## Expected Results Summary

| Check | Expected Result |
|-------|------------------|
| Script Structure | All core scripts exist |
| Error Handling | All scripts have error handling |
| Cross-Platform Parity | All scripts have PS1/.sh pairs |
| Checksum Support | Download functions support checksums |
| Logging | Logging functions exist |
| State Management | State functions exist |
| Test Suite | All tests pass |
| Hash Generation | Hash file created successfully |
| Quality Report | Report exists with check items |

---

## Next Steps

After completing all verification steps:

1. Review `phase0-quality-report.md` for detailed findings
2. Review `phase0-FINAL_REPORT.md` for summary
3. Address any identified gaps
4. Run smoke tests: `scripts/bootstrap/verify/smoke-test.ps1` / `.sh`
5. Complete triple-verification Pass B/C

---

**Last Updated**: 2025-01-27
