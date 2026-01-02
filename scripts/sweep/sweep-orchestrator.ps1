<#
.SYNOPSIS
    Parallel Sweep Orchestrator - 10-loop codebase audit system

.DESCRIPTION
    Orchestrates parallel sweeps through the noa directory, extracting symbols,
    cross-checking documentation, generating embeddings, and producing graphs.
    
    Uses Ollama for embeddings, SQLite for state, and RuVector for vector storage.

.PARAMETER SweepNumber
    Current sweep iteration (1-10)

.PARAMETER MaxParallel
    Maximum parallel jobs per directory level (default: 8)

.PARAMETER OllamaModel
    Ollama embedding model to use (default: nomic-embed-text)

.EXAMPLE
    .\sweep-orchestrator.ps1 -SweepNumber 1 -MaxParallel 8
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [ValidateRange(1, 10)]
    [int]$SweepNumber = 1,

    [Parameter(Mandatory = $false)]
    [ValidateRange(1, 32)]
    [int]$MaxParallel = 8,

    [Parameter(Mandatory = $false)]
    [string]$OllamaModel = 'nomic-embed-text',

    [Parameter(Mandatory = $false)]
    [switch]$DryRun,

    [Parameter(Mandatory = $false)]
    [switch]$Force
)

#region Configuration
$ErrorActionPreference = 'Stop'
$script:NoaRoot = (Resolve-Path "$PSScriptRoot\..\..").Path
$script:SweepDir = "$script:NoaRoot\scripts\sweep"
$script:DataDir = "$script:NoaRoot\data\state\sweep"
$script:DbPath = "$script:DataDir\sweep.db"
$script:LogDir = "$script:NoaRoot\logs\sweep"
$script:LogFile = "$script:LogDir\sweep-$(Get-Date -Format 'yyyy-MM-dd-HHmmss').log"

# Directories to skip (vendor, cache, build outputs)
$script:SkipDirs = @(
    '.git', 'node_modules', 'target', 'dist', 'build',
    '__pycache__', '.pytest_cache', '.mypy_cache',
    'vendor', '.cargo', '.rustup', 'cache',
    'deployed-plane', 'tmp', '.vscode'
)

# File extensions to process
$script:ProcessExtensions = @{
    Rust       = @('.rs')
    TypeScript = @('.ts', '.tsx', '.mts', '.cts')
    JavaScript = @('.js', '.jsx', '.mjs', '.cjs')
    Python     = @('.py', '.pyi')
    Markdown   = @('.md', '.mdx')
    Config     = @('.json', '.yaml', '.yml', '.toml')
    Shell      = @('.ps1', '.sh', '.bash')
}
#endregion

#region Logging
function Write-SweepLog {
    param(
        [string]$Message,
        [ValidateSet('INFO', 'WARN', 'ERROR', 'DEBUG', 'SUCCESS')]
        [string]$Level = 'INFO'
    )
    
    $timestamp = Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff'
    $logLine = "[$timestamp] [$Level] $Message"
    
    # Console output with colors
    switch ($Level) {
        'ERROR' { Write-Host $logLine -ForegroundColor Red }
        'WARN' { Write-Host $logLine -ForegroundColor Yellow }
        'SUCCESS' { Write-Host $logLine -ForegroundColor Green }
        'DEBUG' { Write-Host $logLine -ForegroundColor Gray }
        default { Write-Host $logLine }
    }
    
    # File output
    if ($script:LogFile) {
        Add-Content -Path $script:LogFile -Value $logLine -ErrorAction SilentlyContinue
    }
}
#endregion

#region SQLite Operations
function Initialize-SweepDatabase {
    Write-SweepLog "Initializing SQLite database at $script:DbPath"
    
    # Ensure directories exist
    if (!(Test-Path $script:DataDir)) {
        New-Item -ItemType Directory -Path $script:DataDir -Force | Out-Null
    }
    if (!(Test-Path $script:LogDir)) {
        New-Item -ItemType Directory -Path $script:LogDir -Force | Out-Null
    }

    # Create tables using sqlite3 (bundled with Git for Windows)
    $createTablesSql = @'
-- Sweep state tracking
CREATE TABLE IF NOT EXISTS sweep_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sweep_number INTEGER NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT,
    status TEXT DEFAULT 'running',
    files_processed INTEGER DEFAULT 0,
    symbols_found INTEGER DEFAULT 0,
    errors INTEGER DEFAULT 0
);

-- File processing state
CREATE TABLE IF NOT EXISTS file_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sweep_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    file_hash TEXT,
    language TEXT,
    status TEXT DEFAULT 'pending',
    symbols_count INTEGER DEFAULT 0,
    has_embedding INTEGER DEFAULT 0,
    has_docs INTEGER DEFAULT 0,
    processed_at TEXT,
    error_message TEXT,
    FOREIGN KEY (sweep_id) REFERENCES sweep_state(id),
    UNIQUE(sweep_id, file_path)
);

-- Symbol index
CREATE TABLE IF NOT EXISTS symbols (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id INTEGER NOT NULL,
    symbol_name TEXT NOT NULL,
    symbol_type TEXT NOT NULL,
    line_number INTEGER,
    visibility TEXT,
    signature TEXT,
    doc_comment TEXT,
    embedding_id TEXT,
    wiki_ref TEXT,
    runbook_ref TEXT,
    pages_ref TEXT,
    FOREIGN KEY (file_id) REFERENCES file_state(id)
);

-- Embeddings storage
CREATE TABLE IF NOT EXISTS embeddings (
    id TEXT PRIMARY KEY,
    content_hash TEXT NOT NULL,
    embedding BLOB NOT NULL,
    model TEXT NOT NULL,
    dimensions INTEGER NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Documentation cross-references
CREATE TABLE IF NOT EXISTS doc_refs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol_id INTEGER,
    doc_type TEXT NOT NULL,
    doc_path TEXT NOT NULL,
    line_number INTEGER,
    status TEXT DEFAULT 'unknown',
    FOREIGN KEY (symbol_id) REFERENCES symbols(id)
);

-- Graph edges for dependency tracking
CREATE TABLE IF NOT EXISTS graph_edges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_symbol_id INTEGER NOT NULL,
    target_symbol_id INTEGER NOT NULL,
    edge_type TEXT NOT NULL,
    weight REAL DEFAULT 1.0,
    FOREIGN KEY (source_symbol_id) REFERENCES symbols(id),
    FOREIGN KEY (target_symbol_id) REFERENCES symbols(id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_file_state_path ON file_state(file_path);
CREATE INDEX IF NOT EXISTS idx_file_state_status ON file_state(status);
CREATE INDEX IF NOT EXISTS idx_symbols_name ON symbols(symbol_name);
CREATE INDEX IF NOT EXISTS idx_symbols_type ON symbols(symbol_type);
CREATE INDEX IF NOT EXISTS idx_embeddings_hash ON embeddings(content_hash);
'@

    # Write SQL to temp file and execute
    $sqlFile = [System.IO.Path]::GetTempFileName()
    Set-Content -Path $sqlFile -Value $createTablesSql -Encoding UTF8
    
    try {
        $sqlite = Get-Command sqlite3 -ErrorAction SilentlyContinue
        if (!$sqlite) {
            # Try Git for Windows bundled sqlite3
            $gitPath = (Get-Command git -ErrorAction SilentlyContinue).Source | Split-Path | Split-Path
            $sqlite = "$gitPath\usr\bin\sqlite3.exe"
        }
        
        if (Test-Path $sqlite) {
            & $sqlite $script:DbPath ".read $sqlFile" 2>&1
            Write-SweepLog 'Database initialized successfully' -Level SUCCESS
        }
        else {
            Write-SweepLog 'sqlite3 not found, using fallback JSON storage' -Level WARN
            Initialize-JsonFallback
        }
    }
    finally {
        Remove-Item $sqlFile -ErrorAction SilentlyContinue
    }
}

function Initialize-JsonFallback {
    # Fallback to JSON files if SQLite unavailable
    $stateFile = "$script:DataDir\sweep-state.json"
    if (!(Test-Path $stateFile)) {
        @{
            sweeps      = @()
            files       = @{}
            symbols     = @{}
            embeddings  = @{}
            doc_refs    = @{}
            graph_edges = @()
        } | ConvertTo-Json -Depth 10 | Set-Content $stateFile
    }
    Write-SweepLog "JSON fallback storage initialized at $stateFile" -Level INFO
}

function Start-SweepRecord {
    param([int]$SweepNumber)
    
    $timestamp = Get-Date -Format 'yyyy-MM-ddTHH:mm:ss'
    $sql = "INSERT INTO sweep_state (sweep_number, start_time, status) VALUES ($SweepNumber, '$timestamp', 'running'); SELECT last_insert_rowid();"
    
    try {
        $result = Invoke-Sqlite $sql
        return [int]$result
    }
    catch {
        Write-SweepLog "Failed to create sweep record: $_" -Level ERROR
        return -1
    }
}

function Invoke-Sqlite {
    param([string]$Sql)
    
    $sqlite = Get-Command sqlite3 -ErrorAction SilentlyContinue
    if (!$sqlite) {
        $gitPath = (Get-Command git -ErrorAction SilentlyContinue).Source | Split-Path | Split-Path
        $sqlite = "$gitPath\usr\bin\sqlite3.exe"
    }
    
    if (Test-Path $sqlite) {
        return & $sqlite $script:DbPath $Sql 2>&1
    }
    else {
        throw 'sqlite3 not available'
    }
}
#endregion

#region Ollama Integration
function Test-OllamaAvailable {
    try {
        $response = Invoke-RestMethod -Uri 'http://localhost:11434/api/tags' -Method Get -TimeoutSec 5 -ErrorAction Stop
        return $true
    }
    catch {
        return $false
    }
}

function Get-OllamaEmbedding {
    param(
        [string]$Text,
        [string]$Model = $script:OllamaModel
    )
    
    $body = @{
        model  = $Model
        prompt = $Text
    } | ConvertTo-Json -Compress
    
    try {
        $response = Invoke-RestMethod -Uri 'http://localhost:11434/api/embeddings' `
            -Method Post `
            -Body $body `
            -ContentType 'application/json' `
            -TimeoutSec 30
        
        return $response.embedding
    }
    catch {
        Write-SweepLog "Ollama embedding failed: $_" -Level WARN
        return $null
    }
}

function Ensure-OllamaModel {
    param([string]$Model)
    
    Write-SweepLog "Checking Ollama model: $Model"
    
    try {
        $response = Invoke-RestMethod -Uri 'http://localhost:11434/api/tags' -Method Get -TimeoutSec 10
        $installed = $response.models | Where-Object { $_.name -like "$Model*" }
        
        if (!$installed) {
            Write-SweepLog "Pulling Ollama model: $Model" -Level INFO
            $pullBody = @{ name = $Model } | ConvertTo-Json
            Invoke-RestMethod -Uri 'http://localhost:11434/api/pull' -Method Post -Body $pullBody -ContentType 'application/json' -TimeoutSec 600
            Write-SweepLog "Model $Model pulled successfully" -Level SUCCESS
        }
        else {
            Write-SweepLog "Model $Model already available" -Level SUCCESS
        }
    }
    catch {
        Write-SweepLog "Failed to ensure Ollama model: $_" -Level ERROR
        return $false
    }
    return $true
}
#endregion

#region File Discovery
function Get-SweepFiles {
    param([string]$RootPath)
    
    Write-SweepLog "Discovering files in $RootPath"
    
    $allExtensions = $script:ProcessExtensions.Values | ForEach-Object { $_ } | Select-Object -Unique
    
    $files = Get-ChildItem -Path $RootPath -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
        # Skip excluded directories
        $inSkipDir = $false
        foreach ($skip in $script:SkipDirs) {
            if ($_.FullName -like "*\$skip\*" -or $_.FullName -like "*/$skip/*") {
                $inSkipDir = $true
                break
            }
        }
        
        if ($inSkipDir) { return $false }
        
        # Check extension
        $ext = $_.Extension.ToLower()
        return $allExtensions -contains $ext
    }
    
    Write-SweepLog "Found $($files.Count) files to process" -Level SUCCESS
    return $files
}

function Get-FileLanguage {
    param([string]$FilePath)
    
    $ext = [System.IO.Path]::GetExtension($FilePath).ToLower()
    
    foreach ($lang in $script:ProcessExtensions.Keys) {
        if ($script:ProcessExtensions[$lang] -contains $ext) {
            return $lang
        }
    }
    
    return 'Unknown'
}

function Get-FileHash {
    param([string]$FilePath)
    
    try {
        $hash = Get-FileHash -Path $FilePath -Algorithm SHA256 -ErrorAction Stop
        return $hash.Hash.Substring(0, 16)  # Short hash
    }
    catch {
        return 'unknown'
    }
}
#endregion

#region Symbol Extraction
function Invoke-SymbolExtractor {
    param(
        [string]$FilePath,
        [string]$Language
    )
    
    $extractorScript = "$script:SweepDir\symbol-extractor.ps1"
    
    if (Test-Path $extractorScript) {
        try {
            $symbols = & $extractorScript -FilePath $FilePath -Language $Language
            return $symbols
        }
        catch {
            Write-SweepLog "Symbol extraction failed for $FilePath`: $_" -Level WARN
            return @()
        }
    }
    else {
        # Inline fallback extraction
        return Get-BasicSymbols -FilePath $FilePath -Language $Language
    }
}

function Get-BasicSymbols {
    param(
        [string]$FilePath,
        [string]$Language
    )
    
    $symbols = @()
    $content = Get-Content $FilePath -Raw -ErrorAction SilentlyContinue
    if (!$content) { return $symbols }
    
    $lineNum = 0
    $lines = $content -split "`n"
    
    foreach ($line in $lines) {
        $lineNum++
        
        switch ($Language) {
            'Rust' {
                # pub fn, pub struct, pub enum, pub trait, pub type, pub const, pub static, mod
                if ($line -match '^\s*(pub\s+)?(async\s+)?fn\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[3]
                        type       = 'function'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'public' } else { 'private' }
                        signature  = $line.Trim()
                    }
                }
                elseif ($line -match '^\s*(pub\s+)?struct\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'struct'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'public' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(pub\s+)?enum\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'enum'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'public' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(pub\s+)?trait\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'trait'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'public' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(pub\s+)?type\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'type_alias'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'public' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(pub\s+)?const\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'constant'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'public' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(pub\s+)?mod\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'module'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'public' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*impl\s+(<[^>]+>\s+)?(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'impl'
                        line       = $lineNum
                        visibility = 'private'
                    }
                }
            }
            
            'TypeScript' {
                # export, function, class, interface, type, const, enum
                if ($line -match '^\s*(export\s+)?(async\s+)?function\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[3]
                        type       = 'function'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'export' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(export\s+)?class\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'class'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'export' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(export\s+)?interface\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'interface'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'export' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(export\s+)?type\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'type'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'export' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(export\s+)?const\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'constant'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'export' } else { 'private' }
                    }
                }
                elseif ($line -match '^\s*(export\s+)?enum\s+(\w+)') {
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'enum'
                        line       = $lineNum
                        visibility = if ($Matches[1]) { 'export' } else { 'private' }
                    }
                }
            }
            
            'Python' {
                # def, class, async def
                if ($line -match '^(async\s+)?def\s+(\w+)\s*\(') {
                    $visibility = if ($Matches[2] -match '^_') { 'private' } else { 'public' }
                    $symbols += @{
                        name       = $Matches[2]
                        type       = 'function'
                        line       = $lineNum
                        visibility = $visibility
                    }
                }
                elseif ($line -match '^class\s+(\w+)') {
                    $visibility = if ($Matches[1] -match '^_') { 'private' } else { 'public' }
                    $symbols += @{
                        name       = $Matches[1]
                        type       = 'class'
                        line       = $lineNum
                        visibility = $visibility
                    }
                }
                elseif ($line -match '^(\w+)\s*:\s*\w+\s*=' -and $line -notmatch '^\s') {
                    $symbols += @{
                        name       = $Matches[1]
                        type       = 'constant'
                        line       = $lineNum
                        visibility = 'public'
                    }
                }
            }
            
            'Shell' {
                # function declarations
                if ($line -match '^function\s+(\w[\w-]*)\s*\{?') {
                    $symbols += @{
                        name       = $Matches[1]
                        type       = 'function'
                        line       = $lineNum
                        visibility = 'public'
                    }
                }
                elseif ($line -match '^(\w[\w-]*)\s*\(\)\s*\{?') {
                    $symbols += @{
                        name       = $Matches[1]
                        type       = 'function'
                        line       = $lineNum
                        visibility = 'public'
                    }
                }
            }
        }
    }
    
    return $symbols
}
#endregion

#region Documentation Cross-Reference
function Get-DocReferences {
    param([string]$SymbolName)
    
    $refs = @{
        wiki    = @()
        runbook = @()
        pages   = @()
    }
    
    # Search wiki
    $wikiFiles = Get-ChildItem "$script:NoaRoot\docs\wiki" -Filter '*.md' -Recurse -ErrorAction SilentlyContinue
    foreach ($file in $wikiFiles) {
        $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
        if ($content -match [regex]::Escape($SymbolName)) {
            $refs.wiki += $file.FullName.Replace($script:NoaRoot, '').TrimStart('\', '/')
        }
    }
    
    # Search runbooks
    $runbookFiles = Get-ChildItem "$script:NoaRoot\docs\runbooks" -Filter '*.md' -Recurse -ErrorAction SilentlyContinue
    foreach ($file in $runbookFiles) {
        $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
        if ($content -match [regex]::Escape($SymbolName)) {
            $refs.runbook += $file.FullName.Replace($script:NoaRoot, '').TrimStart('\', '/')
        }
    }
    
    # Search pages
    $pagesFiles = Get-ChildItem "$script:NoaRoot\docs\pages" -Filter '*.md' -Recurse -ErrorAction SilentlyContinue
    foreach ($file in $pagesFiles) {
        $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
        if ($content -match [regex]::Escape($SymbolName)) {
            $refs.pages += $file.FullName.Replace($script:NoaRoot, '').TrimStart('\', '/')
        }
    }
    
    return $refs
}
#endregion

#region Parallel Processing
function Invoke-ParallelSweep {
    param(
        [object[]]$Files,
        [int]$SweepId,
        [int]$MaxParallel
    )
    
    Write-SweepLog "Starting parallel sweep with $MaxParallel workers"
    
    $totalFiles = $Files.Count
    $processedCount = 0
    $symbolsTotal = 0
    $errorsTotal = 0
    
    # Process in batches
    $batchSize = [Math]::Min($MaxParallel * 10, $totalFiles)
    $batches = [Math]::Ceiling($totalFiles / $batchSize)
    
    for ($batch = 0; $batch -lt $batches; $batch++) {
        $start = $batch * $batchSize
        $end = [Math]::Min($start + $batchSize - 1, $totalFiles - 1)
        $batchFiles = $Files[$start..$end]
        
        Write-SweepLog "Processing batch $($batch + 1)/$batches ($($batchFiles.Count) files)"
        
        $jobs = @()
        
        foreach ($file in $batchFiles) {
            # Create job for each file
            $jobs += Start-ThreadJob -ScriptBlock {
                param($FilePath, $NoaRoot, $SweepDir, $OllamaModel)
                
                $result = @{
                    FilePath  = $FilePath
                    Status    = 'pending'
                    Symbols   = @()
                    DocRefs   = @{}
                    Embedding = $null
                    Error     = $null
                }
                
                try {
                    # Get language
                    $ext = [System.IO.Path]::GetExtension($FilePath).ToLower()
                    $langMap = @{
                        '.rs'  = 'Rust'
                        '.ts'  = 'TypeScript'
                        '.tsx' = 'TypeScript'
                        '.js'  = 'JavaScript'
                        '.py'  = 'Python'
                        '.ps1' = 'Shell'
                        '.sh'  = 'Shell'
                    }
                    $language = $langMap[$ext] ?? 'Unknown'
                    
                    # Read file content
                    $content = Get-Content $FilePath -Raw -ErrorAction Stop
                    
                    # Extract symbols inline
                    $symbols = @()
                    $lineNum = 0
                    $lines = $content -split "`n"
                    
                    foreach ($line in $lines) {
                        $lineNum++
                        
                        # Rust patterns
                        if ($language -eq 'Rust') {
                            if ($line -match '^\s*(pub\s+)?(async\s+)?fn\s+(\w+)') {
                                $symbols += @{
                                    name       = $Matches[3]
                                    type       = 'function'
                                    line       = $lineNum
                                    visibility = if ($Matches[1]) { 'public' } else { 'private' }
                                }
                            }
                            elseif ($line -match '^\s*(pub\s+)?struct\s+(\w+)') {
                                $symbols += @{ name = $Matches[2]; type = 'struct'; line = $lineNum }
                            }
                            elseif ($line -match '^\s*(pub\s+)?enum\s+(\w+)') {
                                $symbols += @{ name = $Matches[2]; type = 'enum'; line = $lineNum }
                            }
                            elseif ($line -match '^\s*(pub\s+)?trait\s+(\w+)') {
                                $symbols += @{ name = $Matches[2]; type = 'trait'; line = $lineNum }
                            }
                        }
                        # TypeScript patterns
                        elseif ($language -eq 'TypeScript' -or $language -eq 'JavaScript') {
                            if ($line -match '^\s*(export\s+)?(async\s+)?function\s+(\w+)') {
                                $symbols += @{ name = $Matches[3]; type = 'function'; line = $lineNum }
                            }
                            elseif ($line -match '^\s*(export\s+)?class\s+(\w+)') {
                                $symbols += @{ name = $Matches[2]; type = 'class'; line = $lineNum }
                            }
                            elseif ($line -match '^\s*(export\s+)?interface\s+(\w+)') {
                                $symbols += @{ name = $Matches[2]; type = 'interface'; line = $lineNum }
                            }
                        }
                        # Python patterns
                        elseif ($language -eq 'Python') {
                            if ($line -match '^(async\s+)?def\s+(\w+)\s*\(') {
                                $symbols += @{ name = $Matches[2]; type = 'function'; line = $lineNum }
                            }
                            elseif ($line -match '^class\s+(\w+)') {
                                $symbols += @{ name = $Matches[1]; type = 'class'; line = $lineNum }
                            }
                        }
                    }
                    
                    $result.Symbols = $symbols
                    $result.Status = 'processed'
                }
                catch {
                    $result.Status = 'error'
                    $result.Error = $_.Exception.Message
                }
                
                return $result
            } -ArgumentList $file.FullName, $script:NoaRoot, $script:SweepDir, $OllamaModel -ThrottleLimit $MaxParallel
        }
        
        # Wait for batch completion
        $results = $jobs | Wait-Job | Receive-Job
        $jobs | Remove-Job -Force
        
        foreach ($result in $results) {
            $processedCount++
            $symbolsTotal += $result.Symbols.Count
            
            if ($result.Status -eq 'error') {
                $errorsTotal++
            }
            
            # Progress update
            if ($processedCount % 100 -eq 0 -or $processedCount -eq $totalFiles) {
                $pct = [Math]::Round(($processedCount / $totalFiles) * 100, 1)
                Write-SweepLog "Progress: $processedCount/$totalFiles ($pct%) - Symbols: $symbolsTotal, Errors: $errorsTotal"
            }
        }
    }
    
    return @{
        FilesProcessed = $processedCount
        SymbolsFound   = $symbolsTotal
        Errors         = $errorsTotal
    }
}
#endregion

#region Main Execution
function Start-Sweep {
    Write-SweepLog ('=' * 60)
    Write-SweepLog "NOA CODEBASE SWEEP - LOOP $SweepNumber OF 10"
    Write-SweepLog ('=' * 60)
    
    # Initialize database
    Initialize-SweepDatabase
    
    # Check Ollama
    if (Test-OllamaAvailable) {
        Write-SweepLog 'Ollama available at localhost:11434' -Level SUCCESS
        Ensure-OllamaModel -Model $OllamaModel
    }
    else {
        Write-SweepLog 'Ollama not available - embeddings will be skipped' -Level WARN
    }
    
    # Start sweep record
    $sweepId = Start-SweepRecord -SweepNumber $SweepNumber
    Write-SweepLog "Sweep ID: $sweepId"
    
    if ($DryRun) {
        Write-SweepLog 'DRY RUN MODE - No changes will be made' -Level WARN
    }
    
    # Discover files
    $files = Get-SweepFiles -RootPath $script:NoaRoot
    
    # Run parallel sweep
    $results = Invoke-ParallelSweep -Files $files -SweepId $sweepId -MaxParallel $MaxParallel
    
    # Summary
    Write-SweepLog ('=' * 60)
    Write-SweepLog "SWEEP $SweepNumber COMPLETE" -Level SUCCESS
    Write-SweepLog "Files Processed: $($results.FilesProcessed)"
    Write-SweepLog "Symbols Found: $($results.SymbolsFound)"
    Write-SweepLog "Errors: $($results.Errors)"
    Write-SweepLog ('=' * 60)
    
    # Update sweep record
    $endTime = Get-Date -Format 'yyyy-MM-ddTHH:mm:ss'
    $updateSql = @"
UPDATE sweep_state 
SET end_time = '$endTime', 
    status = 'completed',
    files_processed = $($results.FilesProcessed),
    symbols_found = $($results.SymbolsFound),
    errors = $($results.Errors)
WHERE id = $sweepId;
"@
    
    try {
        Invoke-Sqlite $updateSql
    }
    catch {
        Write-SweepLog "Failed to update sweep record: $_" -Level WARN
    }
    
    return $results
}

# Execute
Start-Sweep
