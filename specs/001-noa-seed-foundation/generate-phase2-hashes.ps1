# Generate SHA-256 hashes for Phase 2 key files
# CHK010, CHK021: SHA-256 hashes for all key Phase 2 artifacts

$noaRoot = $env:NOA_ROOT
if (-not $noaRoot) {
    $noaRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
}

$specDir = Join-Path $noaRoot "specs\001-noa-seed-foundation"
$outputFile = Join-Path $specDir "HASHES-PHASE2.txt"

# Phase 2 key files
$phase2Files = @(
    # Storage Components Setup (T018a-T018f)
    "data\memory",
    "data\knowledge",
    "data\embeddings",
    "data\artifacts",
    "containers\oci\registry.yaml",
    "config\minio.yaml",
    "config\database.yaml",
    "config\qdrant.yaml",
    "config\quickwit.yaml",

    # Database Schema (T018g-T037)
    "init\migrations\001_initial.sql",
    "init\migrations\002_indexes.sql",
    "init\migrations\003_vectors.sql",
    "init\migrations\pg\001_pgvector.sql",

    # CSV Export & Schemas (T041-T045)
    "sys\core\src\export\csv_export.rs",
    "config\schemas\csv\agent_directory.yaml",
    "config\schemas\csv\task_tables.yaml",
    "config\schemas\csv\claims_evidence.yaml",
    "config\schemas\csv\metrics_traces.yaml",

    # Configuration Standards (T046-T049)
    "config\schemas\config_schema.json",
    "sys\core\src\config\validator.rs",
    "sys\core\src\config\lineage.rs",

    # Rust Core Foundation (T050-T055)
    "sys\core\src\error.rs",
    "sys\core\src\config\mod.rs",
    "sys\core\src\logging.rs",
    "sys\core\src\db\pool.rs",
    "sys\core\src\db\repository.rs",
    "sys\core\src\db\migrations.rs",
    "sys\core\src\db\mod.rs",

    # API Foundation (T056-T060)
    "sys\core\src\api\server.rs",
    "sys\core\src\api\routes\health.rs",
    "sys\core\src\api\middleware\validation.rs",
    "sys\core\src\api\middleware\logging.rs",
    "sys\core\src\api\middleware\telemetry.rs",

    # CLI Foundation (T061-T067)
    "sys\core\src\main.rs",
    "sys\core\src\cli\init.rs",
    "sys\core\src\cli\start.rs",
    "sys\core\src\cli\status.rs",
    "sys\core\src\cli\stop.rs",
    "sys\core\src\cli\db.rs",

    # Observability Foundation (T068-T071)
    "sys\core\src\observability\logging.rs",
    "sys\core\src\observability\telemetry.rs",
    "sys\core\src\observability\metrics.rs",
    "config\observability.yaml",

    # Smoke Tests
    "scripts\test\smoke-test-phase2.sh",
    "scripts\test\smoke-test-phase2.ps1"
)

$hashes = @()
$missing = @()

foreach ($file in $phase2Files) {
    $fullPath = Join-Path $noaRoot $file

    if (Test-Path $fullPath -PathType Leaf) {
        $hash = Get-FileHash $fullPath -Algorithm SHA256
        $relativePath = $file.Replace('\', '/')
        $hashes += "$($hash.Hash)  $relativePath"
    } elseif (Test-Path $fullPath -PathType Container) {
        # For directories, hash all files inside
        $dirFiles = Get-ChildItem -Path $fullPath -Recurse -File
        foreach ($dirFile in $dirFiles) {
            $hash = Get-FileHash $dirFile.FullName -Algorithm SHA256
            $relativePath = $dirFile.FullName.Replace($noaRoot + '\', '').Replace('\', '/')
            $hashes += "$($hash.Hash)  $relativePath"
        }
    } else {
        $missing += $file
    }
}

# Sort hashes
$hashes = $hashes | Sort-Object

# Write output
$hashes | Out-File -FilePath $outputFile -Encoding utf8

Write-Host "Generated HASHES-PHASE2.txt with $($hashes.Count) files" -ForegroundColor Green

if ($missing.Count -gt 0) {
    Write-Host "`nWarning: $($missing.Count) files/directories not found:" -ForegroundColor Yellow
    $missing | ForEach-Object { Write-Host "  - $_" -ForegroundColor Yellow }
}

Write-Host "`nOutput file: $outputFile" -ForegroundColor Cyan

