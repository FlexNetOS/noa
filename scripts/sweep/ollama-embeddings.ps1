<#
.SYNOPSIS
    Ollama Embeddings Generator - Creates vector embeddings using Ollama

.DESCRIPTION
    Generates vector embeddings for code symbols and documentation using
    Ollama's local embedding models (nomic-embed-text, mxbai-embed-large, etc.)

.PARAMETER Content
    Text content to embed

.PARAMETER Model
    Ollama embedding model (default: nomic-embed-text)

.PARAMETER BatchSize
    Number of items to embed in parallel

.EXAMPLE
    .\ollama-embeddings.ps1 -Content "function processData" -Model nomic-embed-text
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false, ValueFromPipeline = $true)]
    [string]$Content,

    [Parameter(Mandatory = $false)]
    [string[]]$ContentBatch,

    [Parameter(Mandatory = $false)]
    [string]$InputFile,

    [Parameter(Mandatory = $false)]
    [ValidateSet('nomic-embed-text', 'mxbai-embed-large', 'all-minilm', 'snowflake-arctic-embed')]
    [string]$Model = 'nomic-embed-text',

    [Parameter(Mandatory = $false)]
    [string]$OutputFile,

    [Parameter(Mandatory = $false)]
    [int]$BatchSize = 10,

    [Parameter(Mandatory = $false)]
    [string]$OllamaHost = "http://localhost:11434"
)

$ErrorActionPreference = "Stop"

#region Ollama Client
class OllamaClient {
    [string]$Host
    [string]$Model
    [hashtable]$Cache
    [int]$CacheHits
    [int]$ApiCalls
    
    OllamaClient([string]$host, [string]$model) {
        $this.Host = $host
        $this.Model = $model
        $this.Cache = @{}
        $this.CacheHits = 0
        $this.ApiCalls = 0
    }
    
    [bool] TestConnection() {
        try {
            $response = Invoke-RestMethod -Uri "$($this.Host)/api/tags" -Method Get -TimeoutSec 5
            return $true
        }
        catch {
            return $false
        }
    }
    
    [hashtable] GetModels() {
        try {
            $response = Invoke-RestMethod -Uri "$($this.Host)/api/tags" -Method Get -TimeoutSec 10
            return @{
                success = $true
                models  = $response.models
            }
        }
        catch {
            return @{
                success = $false
                error   = $_.Exception.Message
            }
        }
    }
    
    [hashtable] PullModel([string]$modelName) {
        Write-Host "Pulling model: $modelName (this may take a while)..."
        
        try {
            $body = @{ name = $modelName } | ConvertTo-Json
            
            # Stream the response
            $request = [System.Net.WebRequest]::Create("$($this.Host)/api/pull")
            $request.Method = "POST"
            $request.ContentType = "application/json"
            
            $bytes = [System.Text.Encoding]::UTF8.GetBytes($body)
            $request.ContentLength = $bytes.Length
            
            $stream = $request.GetRequestStream()
            $stream.Write($bytes, 0, $bytes.Length)
            $stream.Close()
            
            $response = $request.GetResponse()
            $reader = New-Object System.IO.StreamReader($response.GetResponseStream())
            
            while (!$reader.EndOfStream) {
                $line = $reader.ReadLine()
                $json = $line | ConvertFrom-Json
                if ($json.status) {
                    Write-Host "  $($json.status)" -NoNewline
                    if ($json.completed -and $json.total) {
                        $pct = [math]::Round(($json.completed / $json.total) * 100, 1)
                        Write-Host " ($pct%)" -NoNewline
                    }
                    Write-Host ""
                }
            }
            
            $reader.Close()
            $response.Close()
            
            return @{ success = $true }
        }
        catch {
            return @{
                success = $false
                error   = $_.Exception.Message
            }
        }
    }
    
    [hashtable] EnsureModel() {
        $models = $this.GetModels()
        
        if (!$models.success) {
            return @{
                success = $false
                error   = "Failed to get model list: $($models.error)"
            }
        }
        
        $hasModel = $models.models | Where-Object { $_.name -like "$($this.Model)*" }
        
        if (!$hasModel) {
            return $this.PullModel($this.Model)
        }
        
        return @{ success = $true }
    }
    
    [float[]] GetEmbedding([string]$text) {
        # Check cache
        $hash = $this.GetContentHash($text)
        if ($this.Cache[$hash]) {
            $this.CacheHits++
            return $this.Cache[$hash]
        }
        
        $body = @{
            model  = $this.Model
            prompt = $text
        } | ConvertTo-Json -Compress
        
        try {
            $this.ApiCalls++
            $response = Invoke-RestMethod -Uri "$($this.Host)/api/embeddings" `
                -Method Post `
                -Body $body `
                -ContentType "application/json" `
                -TimeoutSec 60
            
            $embedding = [float[]]$response.embedding
            
            # Cache result
            $this.Cache[$hash] = $embedding
            
            return $embedding
        }
        catch {
            Write-Warning "Embedding failed: $($_.Exception.Message)"
            return $null
        }
    }
    
    [hashtable[]] GetEmbeddingsBatch([string[]]$texts) {
        $results = @()
        
        foreach ($text in $texts) {
            $embedding = $this.GetEmbedding($text)
            $results += @{
                text       = $text.Substring(0, [Math]::Min(100, $text.Length))
                embedding  = $embedding
                dimensions = if ($embedding) { $embedding.Length } else { 0 }
                success    = $embedding -ne $null
            }
        }
        
        return $results
    }
    
    [string] GetContentHash([string]$content) {
        $bytes = [System.Text.Encoding]::UTF8.GetBytes($content)
        $hash = [System.Security.Cryptography.SHA256]::Create().ComputeHash($bytes)
        return [System.BitConverter]::ToString($hash).Replace("-", "").Substring(0, 16)
    }
    
    [hashtable] GetStats() {
        return @{
            model     = $this.Model
            host      = $this.Host
            cacheSize = $this.Cache.Count
            cacheHits = $this.CacheHits
            apiCalls  = $this.ApiCalls
        }
    }
}
#endregion

#region Symbol Embedding
function Get-SymbolEmbeddingText {
    param([object]$Symbol)
    
    # Create rich text representation for embedding
    $parts = @()
    
    # Symbol type and name
    $parts += "$($Symbol.Type): $($Symbol.Name)"
    
    # Visibility
    if ($Symbol.Visibility) {
        $parts += "visibility: $($Symbol.Visibility)"
    }
    
    # Signature/Parameters
    if ($Symbol.Signature) {
        $parts += "signature: $($Symbol.Signature)"
    }
    elseif ($Symbol.Parameters) {
        $parts += "parameters: $($Symbol.Parameters)"
    }
    
    # Return type
    if ($Symbol.ReturnType) {
        $parts += "returns: $($Symbol.ReturnType)"
    }
    
    # Documentation
    if ($Symbol.DocComment) {
        $parts += "doc: $($Symbol.DocComment)"
    }
    
    # File context
    if ($Symbol.FilePath) {
        $fileName = ($Symbol.FilePath -split "[\\/]")[-1]
        $parts += "file: $fileName"
    }
    
    return $parts -join " | "
}

function Add-SymbolEmbeddings {
    param(
        [object[]]$Symbols,
        [OllamaClient]$Client,
        [int]$BatchSize = 10
    )
    
    $total = $Symbols.Count
    $processed = 0
    $withEmbeddings = 0
    
    Write-Host "Generating embeddings for $total symbols..."
    
    for ($i = 0; $i -lt $total; $i += $BatchSize) {
        $batch = $Symbols[$i..([Math]::Min($i + $BatchSize - 1, $total - 1))]
        
        foreach ($symbol in $batch) {
            $text = Get-SymbolEmbeddingText -Symbol $symbol
            $embedding = $Client.GetEmbedding($text)
            
            if ($embedding) {
                $symbol | Add-Member -NotePropertyName 'Embedding' -NotePropertyValue $embedding -Force
                $symbol | Add-Member -NotePropertyName 'EmbeddingDimensions' -NotePropertyValue $embedding.Length -Force
                $withEmbeddings++
            }
            
            $processed++
        }
        
        # Progress
        $pct = [Math]::Round(($processed / $total) * 100, 1)
        Write-Host "  Progress: $processed/$total ($pct%) - Embeddings: $withEmbeddings"
    }
    
    return $Symbols
}
#endregion

#region File Processing
function Get-FileEmbedding {
    param(
        [string]$FilePath,
        [OllamaClient]$Client
    )
    
    if (!(Test-Path $FilePath)) {
        return $null
    }
    
    $content = Get-Content $FilePath -Raw -ErrorAction SilentlyContinue
    if (!$content) {
        return $null
    }
    
    # Truncate for embedding (most models have token limits)
    $maxChars = 8000
    if ($content.Length -gt $maxChars) {
        $content = $content.Substring(0, $maxChars) + "... [truncated]"
    }
    
    return $Client.GetEmbedding($content)
}
#endregion

#region Main Execution
function Start-EmbeddingGeneration {
    Write-Host "=" * 60
    Write-Host "OLLAMA EMBEDDINGS GENERATOR"
    Write-Host "=" * 60
    Write-Host "Model: $Model"
    Write-Host "Host: $OllamaHost"
    Write-Host ""
    
    # Initialize client
    $client = [OllamaClient]::new($OllamaHost, $Model)
    
    # Test connection
    if (!$client.TestConnection()) {
        Write-Error "Cannot connect to Ollama at $OllamaHost. Please ensure Ollama is running."
        Write-Host "  Start Ollama with: ollama serve"
        exit 1
    }
    
    Write-Host "Connected to Ollama" -ForegroundColor Green
    
    # Ensure model is available
    $modelCheck = $client.EnsureModel()
    if (!$modelCheck.success) {
        Write-Error "Failed to ensure model: $($modelCheck.error)"
        exit 1
    }
    
    Write-Host "Model '$Model' is ready" -ForegroundColor Green
    
    # Process input
    $results = @()
    
    if ($Content) {
        # Single content
        Write-Host "`nEmbedding single content..."
        $embedding = $client.GetEmbedding($Content)
        
        $results += @{
            content    = $Content
            embedding  = $embedding
            dimensions = if ($embedding) { $embedding.Length } else { 0 }
            success    = $embedding -ne $null
        }
    }
    elseif ($ContentBatch) {
        # Batch content
        Write-Host "`nEmbedding batch of $($ContentBatch.Count) items..."
        $results = $client.GetEmbeddingsBatch($ContentBatch)
    }
    elseif ($InputFile -and (Test-Path $InputFile)) {
        # File input
        $ext = [System.IO.Path]::GetExtension($InputFile).ToLower()
        
        if ($ext -eq '.json') {
            Write-Host "`nLoading symbols from JSON..."
            $symbols = Get-Content $InputFile -Raw | ConvertFrom-Json
            $symbols = Add-SymbolEmbeddings -Symbols $symbols -Client $client -BatchSize $BatchSize
            $results = $symbols
        }
        else {
            # Embed file content
            Write-Host "`nEmbedding file content..."
            $embedding = Get-FileEmbedding -FilePath $InputFile -Client $client
            
            $results += @{
                file       = $InputFile
                embedding  = $embedding
                dimensions = if ($embedding) { $embedding.Length } else { 0 }
                success    = $embedding -ne $null
            }
        }
    }
    else {
        Write-Host "No input provided. Use -Content, -ContentBatch, or -InputFile"
        exit 1
    }
    
    # Output results
    if ($OutputFile) {
        $results | ConvertTo-Json -Depth 10 -Compress | Set-Content -Path $OutputFile -Encoding UTF8
        Write-Host "`nResults saved to: $OutputFile" -ForegroundColor Green
    }
    
    # Stats
    $stats = $client.GetStats()
    Write-Host "`n" + "=" * 60
    Write-Host "STATS"
    Write-Host "=" * 60
    Write-Host "  API Calls: $($stats.apiCalls)"
    Write-Host "  Cache Hits: $($stats.cacheHits)"
    Write-Host "  Cache Size: $($stats.cacheSize)"
    
    return $results
}

# Execute
Start-EmbeddingGeneration
