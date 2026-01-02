<#
.SYNOPSIS
    Symbol Extractor - Extracts all public symbols from source files

.DESCRIPTION
    Uses regex patterns and optional tree-sitter for accurate symbol extraction
    from Rust, TypeScript, JavaScript, Python, and Shell files.

.PARAMETER FilePath
    Path to the source file to analyze

.PARAMETER Language
    Programming language (auto-detected from extension if not provided)

.PARAMETER OutputFormat
    Output format: json, csv, or object (default: object)

.EXAMPLE
    .\symbol-extractor.ps1 -FilePath "src/main.rs" -Language Rust
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$FilePath,

    [Parameter(Mandatory = $false)]
    [ValidateSet('Rust', 'TypeScript', 'JavaScript', 'Python', 'Shell', 'Markdown', 'Config', 'Auto')]
    [string]$Language = 'Auto',

    [Parameter(Mandatory = $false)]
    [ValidateSet('json', 'csv', 'object')]
    [string]$OutputFormat = 'object'
)

#region Language Detection
$script:ExtensionMap = @{
    '.rs'    = 'Rust'
    '.ts'    = 'TypeScript'
    '.tsx'   = 'TypeScript'
    '.mts'   = 'TypeScript'
    '.cts'   = 'TypeScript'
    '.js'    = 'JavaScript'
    '.jsx'   = 'JavaScript'
    '.mjs'   = 'JavaScript'
    '.cjs'   = 'JavaScript'
    '.py'    = 'Python'
    '.pyi'   = 'Python'
    '.ps1'   = 'Shell'
    '.sh'    = 'Shell'
    '.bash'  = 'Shell'
    '.md'    = 'Markdown'
    '.mdx'   = 'Markdown'
    '.json'  = 'Config'
    '.yaml'  = 'Config'
    '.yml'   = 'Config'
    '.toml'  = 'Config'
}

function Get-DetectedLanguage {
    param([string]$Path)
    
    $ext = [System.IO.Path]::GetExtension($Path).ToLower()
    return $script:ExtensionMap[$ext] ?? 'Unknown'
}
#endregion

#region Rust Symbol Extraction
function Get-RustSymbols {
    param([string]$Content, [string]$FilePath)
    
    $symbols = @()
    $lines = $Content -split "`n"
    $lineNum = 0
    $currentDocComment = ""
    $inDocComment = $false
    
    foreach ($line in $lines) {
        $lineNum++
        
        # Track doc comments
        if ($line -match '^\s*///\s*(.*)') {
            $currentDocComment += $Matches[1] + " "
            $inDocComment = $true
            continue
        }
        elseif ($line -match '^\s*//!\s*(.*)') {
            # Module-level doc comment
            continue
        }
        elseif ($inDocComment -and $line -match '^\s*$') {
            # Empty line doesn't break doc comment
            continue
        }
        else {
            $inDocComment = $false
        }
        
        # Public function
        if ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?(async\s+)?fn\s+(\w+)\s*(<[^>]*>)?\s*\(([^)]*)\)(\s*->\s*([^{;]+))?') {
            $visibility = if ($Matches[1]) { 
                if ($Matches[2]) { "pub$($Matches[2].Trim())" } else { "pub" }
            } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[4]
                Type       = 'function'
                Line       = $lineNum
                Visibility = $visibility
                Async      = [bool]$Matches[3]
                Generics   = $Matches[5]
                Parameters = $Matches[6].Trim()
                ReturnType = $Matches[8]?.Trim()
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Struct
        elseif ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?struct\s+(\w+)\s*(<[^>]*>)?') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'struct'
                Line       = $lineNum
                Visibility = $visibility
                Generics   = $Matches[4]
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Enum
        elseif ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?enum\s+(\w+)\s*(<[^>]*>)?') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'enum'
                Line       = $lineNum
                Visibility = $visibility
                Generics   = $Matches[4]
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Trait
        elseif ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?trait\s+(\w+)\s*(<[^>]*>)?') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'trait'
                Line       = $lineNum
                Visibility = $visibility
                Generics   = $Matches[4]
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Type alias
        elseif ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?type\s+(\w+)\s*(<[^>]*>)?\s*=\s*(.+);') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'type_alias'
                Line       = $lineNum
                Visibility = $visibility
                Generics   = $Matches[4]
                AliasFor   = $Matches[5].Trim()
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Const
        elseif ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?const\s+(\w+)\s*:\s*([^=]+)\s*=') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'constant'
                Line       = $lineNum
                Visibility = $visibility
                DataType   = $Matches[4].Trim()
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Static
        elseif ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?static\s+(mut\s+)?(\w+)\s*:\s*([^=]+)\s*=') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[4]
                Type       = 'static'
                Line       = $lineNum
                Visibility = $visibility
                Mutable    = [bool]$Matches[3]
                DataType   = $Matches[5].Trim()
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Module
        elseif ($line -match '^\s*(pub(\s*\([^)]*\))?\s+)?mod\s+(\w+)\s*[;{]') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'module'
                Line       = $lineNum
                Visibility = $visibility
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Impl block
        elseif ($line -match '^\s*impl\s*(<[^>]*>)?\s*(\w+)\s*(<[^>]*>)?\s*(for\s+(\w+)\s*(<[^>]*>)?)?\s*\{?') {
            $symbols += [PSCustomObject]@{
                Name       = $Matches[2]
                Type       = 'impl'
                Line       = $lineNum
                Visibility = 'private'
                Generics   = $Matches[1] ?? $Matches[3]
                ForTrait   = $Matches[5]
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        # Macro definition
        elseif ($line -match '^\s*(#\[macro_export\]\s*)?macro_rules!\s+(\w+)') {
            $visibility = if ($Matches[1]) { "pub" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[2]
                Type       = 'macro'
                Line       = $lineNum
                Visibility = $visibility
                DocComment = $currentDocComment.Trim()
                FilePath   = $FilePath
            }
            $currentDocComment = ""
        }
        else {
            $currentDocComment = ""
        }
    }
    
    return $symbols
}
#endregion

#region TypeScript/JavaScript Symbol Extraction
function Get-TypeScriptSymbols {
    param([string]$Content, [string]$FilePath)
    
    $symbols = @()
    $lines = $Content -split "`n"
    $lineNum = 0
    $currentJsDoc = ""
    $inJsDoc = $false
    
    foreach ($line in $lines) {
        $lineNum++
        
        # Track JSDoc comments
        if ($line -match '^\s*/\*\*') {
            $inJsDoc = $true
            $currentJsDoc = ""
            continue
        }
        elseif ($inJsDoc -and $line -match '\*/') {
            $inJsDoc = $false
            continue
        }
        elseif ($inJsDoc) {
            $currentJsDoc += ($line -replace '^\s*\*\s*', '') + " "
            continue
        }
        
        # Function (including arrow functions)
        if ($line -match '^\s*(export\s+)?(default\s+)?(async\s+)?function\s+(\w+)\s*(<[^>]*>)?\s*\(([^)]*)\)(\s*:\s*([^{]+))?\s*\{?') {
            $visibility = if ($Matches[1]) { "export" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[4]
                Type       = 'function'
                Line       = $lineNum
                Visibility = $visibility
                Default    = [bool]$Matches[2]
                Async      = [bool]$Matches[3]
                Generics   = $Matches[5]
                Parameters = $Matches[6].Trim()
                ReturnType = $Matches[8]?.Trim()
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        # Const arrow function
        elseif ($line -match '^\s*(export\s+)?(const|let)\s+(\w+)\s*(<[^>]*>)?\s*=\s*(async\s+)?\([^)]*\)\s*(:\s*[^=]+)?\s*=>') {
            $visibility = if ($Matches[1]) { "export" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'arrow_function'
                Line       = $lineNum
                Visibility = $visibility
                Async      = [bool]$Matches[5]
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        # Class
        elseif ($line -match '^\s*(export\s+)?(default\s+)?(abstract\s+)?class\s+(\w+)\s*(<[^>]*>)?(\s+extends\s+(\w+))?(\s+implements\s+(.+))?\s*\{?') {
            $visibility = if ($Matches[1]) { "export" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[4]
                Type       = 'class'
                Line       = $lineNum
                Visibility = $visibility
                Default    = [bool]$Matches[2]
                Abstract   = [bool]$Matches[3]
                Generics   = $Matches[5]
                Extends    = $Matches[7]
                Implements = $Matches[9]?.Split(',') | ForEach-Object { $_.Trim() }
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        # Interface
        elseif ($line -match '^\s*(export\s+)?interface\s+(\w+)\s*(<[^>]*>)?(\s+extends\s+(.+))?\s*\{?') {
            $visibility = if ($Matches[1]) { "export" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[2]
                Type       = 'interface'
                Line       = $lineNum
                Visibility = $visibility
                Generics   = $Matches[3]
                Extends    = $Matches[5]?.Split(',') | ForEach-Object { $_.Trim() }
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        # Type alias
        elseif ($line -match '^\s*(export\s+)?type\s+(\w+)\s*(<[^>]*>)?\s*=') {
            $visibility = if ($Matches[1]) { "export" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[2]
                Type       = 'type_alias'
                Line       = $lineNum
                Visibility = $visibility
                Generics   = $Matches[3]
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        # Enum
        elseif ($line -match '^\s*(export\s+)?(const\s+)?enum\s+(\w+)\s*\{?') {
            $visibility = if ($Matches[1]) { "export" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'enum'
                Line       = $lineNum
                Visibility = $visibility
                Const      = [bool]$Matches[2]
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        # Const/Let/Var export
        elseif ($line -match '^\s*(export\s+)(const|let|var)\s+(\w+)\s*(:?\s*([^=]+))?\s*=') {
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'constant'
                Line       = $lineNum
                Visibility = 'export'
                Kind       = $Matches[2]
                DataType   = $Matches[5]?.Trim()
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        # Namespace/Module
        elseif ($line -match '^\s*(export\s+)?(declare\s+)?namespace\s+(\w+)\s*\{?') {
            $visibility = if ($Matches[1]) { "export" } else { "private" }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = 'namespace'
                Line       = $lineNum
                Visibility = $visibility
                Declare    = [bool]$Matches[2]
                DocComment = $currentJsDoc.Trim()
                FilePath   = $FilePath
            }
            $currentJsDoc = ""
        }
        else {
            $currentJsDoc = ""
        }
    }
    
    return $symbols
}
#endregion

#region Python Symbol Extraction
function Get-PythonSymbols {
    param([string]$Content, [string]$FilePath)
    
    $symbols = @()
    $lines = $Content -split "`n"
    $lineNum = 0
    $currentDocstring = ""
    $inDocstring = $false
    $docstringQuote = ""
    
    foreach ($line in $lines) {
        $lineNum++
        
        # Track docstrings
        if (!$inDocstring -and $line -match '^\s*("""|\x27\x27\x27)(.*)') {
            $docstringQuote = $Matches[1]
            if ($line -match "$docstringQuote.*$docstringQuote") {
                # Single line docstring
                $currentDocstring = $line -replace "^\s*$docstringQuote\s*" -replace "\s*$docstringQuote\s*$"
            }
            else {
                $inDocstring = $true
                $currentDocstring = $Matches[2]
            }
            continue
        }
        elseif ($inDocstring -and $line -match $docstringQuote) {
            $inDocstring = $false
            continue
        }
        elseif ($inDocstring) {
            $currentDocstring += " " + $line.Trim()
            continue
        }
        
        # Function/Method
        if ($line -match '^(\s*)(async\s+)?def\s+(\w+)\s*\(([^)]*)\)(\s*->\s*(.+))?\s*:') {
            $indent = $Matches[1].Length
            $visibility = if ($Matches[3] -match '^__') { 'dunder' }
            elseif ($Matches[3] -match '^_') { 'private' }
            else { 'public' }
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[3]
                Type       = if ($indent -gt 0) { 'method' } else { 'function' }
                Line       = $lineNum
                Visibility = $visibility
                Async      = [bool]$Matches[2]
                Parameters = $Matches[4].Trim()
                ReturnType = $Matches[6]?.Trim()
                Indent     = $indent
                DocComment = $currentDocstring.Trim()
                FilePath   = $FilePath
            }
            $currentDocstring = ""
        }
        # Class
        elseif ($line -match '^class\s+(\w+)\s*(\([^)]*\))?\s*:') {
            $visibility = if ($Matches[1] -match '^_') { 'private' } else { 'public' }
            $bases = $Matches[2] -replace '^\(' -replace '\)$'
            
            $symbols += [PSCustomObject]@{
                Name       = $Matches[1]
                Type       = 'class'
                Line       = $lineNum
                Visibility = $visibility
                Bases      = $bases?.Split(',') | ForEach-Object { $_.Trim() }
                DocComment = $currentDocstring.Trim()
                FilePath   = $FilePath
            }
            $currentDocstring = ""
        }
        # Module-level constant (UPPER_CASE)
        elseif ($line -match '^([A-Z][A-Z0-9_]*)\s*(:\s*([^=]+))?\s*=') {
            $symbols += [PSCustomObject]@{
                Name       = $Matches[1]
                Type       = 'constant'
                Line       = $lineNum
                Visibility = 'public'
                DataType   = $Matches[3]?.Trim()
                DocComment = $currentDocstring.Trim()
                FilePath   = $FilePath
            }
            $currentDocstring = ""
        }
        # Type alias
        elseif ($line -match '^(\w+)\s*:\s*TypeAlias\s*=') {
            $symbols += [PSCustomObject]@{
                Name       = $Matches[1]
                Type       = 'type_alias'
                Line       = $lineNum
                Visibility = 'public'
                DocComment = $currentDocstring.Trim()
                FilePath   = $FilePath
            }
            $currentDocstring = ""
        }
        # Decorator (for tracking decorated items)
        elseif ($line -match '^@(\w+)') {
            # Just note decorators, next line will have the symbol
            continue
        }
        else {
            $currentDocstring = ""
        }
    }
    
    return $symbols
}
#endregion

#region Shell Symbol Extraction
function Get-ShellSymbols {
    param([string]$Content, [string]$FilePath)
    
    $symbols = @()
    $lines = $Content -split "`n"
    $lineNum = 0
    $currentComment = ""
    
    foreach ($line in $lines) {
        $lineNum++
        
        # Track comments
        if ($line -match '^#\s*(.*)') {
            $currentComment += $Matches[1] + " "
            continue
        }
        
        # PowerShell function
        if ($line -match '^\s*function\s+([\w-]+)\s*\{?') {
            $symbols += [PSCustomObject]@{
                Name       = $Matches[1]
                Type       = 'function'
                Line       = $lineNum
                Visibility = 'public'
                DocComment = $currentComment.Trim()
                FilePath   = $FilePath
            }
            $currentComment = ""
        }
        # Bash function (name() or function name)
        elseif ($line -match '^([\w-]+)\s*\(\)\s*\{?') {
            $symbols += [PSCustomObject]@{
                Name       = $Matches[1]
                Type       = 'function'
                Line       = $lineNum
                Visibility = 'public'
                DocComment = $currentComment.Trim()
                FilePath   = $FilePath
            }
            $currentComment = ""
        }
        # PowerShell param block (for script parameters)
        elseif ($line -match '^\s*\[Parameter\(' -or $line -match '^\s*param\s*\(') {
            # Parameters are tracked differently
            continue
        }
        else {
            $currentComment = ""
        }
    }
    
    return $symbols
}
#endregion

#region Markdown Symbol Extraction
function Get-MarkdownSymbols {
    param([string]$Content, [string]$FilePath)
    
    $symbols = @()
    $lines = $Content -split "`n"
    $lineNum = 0
    
    foreach ($line in $lines) {
        $lineNum++
        
        # Headings
        if ($line -match '^(#{1,6})\s+(.+)') {
            $level = $Matches[1].Length
            $title = $Matches[2].Trim()
            
            $symbols += [PSCustomObject]@{
                Name       = $title
                Type       = "heading_$level"
                Line       = $lineNum
                Visibility = 'public'
                Level      = $level
                FilePath   = $FilePath
            }
        }
        # Code blocks with language
        elseif ($line -match '^```(\w+)') {
            $symbols += [PSCustomObject]@{
                Name       = "codeblock_$($Matches[1])"
                Type       = 'code_block'
                Line       = $lineNum
                Visibility = 'public'
                Language   = $Matches[1]
                FilePath   = $FilePath
            }
        }
        # Links to other files
        elseif ($line -match '\[([^\]]+)\]\(([^)]+)\)') {
            $symbols += [PSCustomObject]@{
                Name       = $Matches[1]
                Type       = 'link'
                Line       = $lineNum
                Visibility = 'public'
                Target     = $Matches[2]
                FilePath   = $FilePath
            }
        }
    }
    
    return $symbols
}
#endregion

#region Main Execution
function Get-FileSymbols {
    param(
        [string]$Path,
        [string]$Lang
    )
    
    if (!(Test-Path $Path)) {
        throw "File not found: $Path"
    }
    
    $content = Get-Content $Path -Raw -ErrorAction Stop
    if (!$content) {
        return @()
    }
    
    if ($Lang -eq 'Auto') {
        $Lang = Get-DetectedLanguage -Path $Path
    }
    
    $symbols = switch ($Lang) {
        'Rust' { Get-RustSymbols -Content $content -FilePath $Path }
        'TypeScript' { Get-TypeScriptSymbols -Content $content -FilePath $Path }
        'JavaScript' { Get-TypeScriptSymbols -Content $content -FilePath $Path }
        'Python' { Get-PythonSymbols -Content $content -FilePath $Path }
        'Shell' { Get-ShellSymbols -Content $content -FilePath $Path }
        'Markdown' { Get-MarkdownSymbols -Content $content -FilePath $Path }
        default { @() }
    }
    
    return $symbols
}

# Execute
try {
    $symbols = Get-FileSymbols -Path $FilePath -Lang $Language
    
    switch ($OutputFormat) {
        'json' {
            $symbols | ConvertTo-Json -Depth 10
        }
        'csv' {
            $symbols | ConvertTo-Csv -NoTypeInformation
        }
        default {
            $symbols
        }
    }
}
catch {
    Write-Error "Symbol extraction failed: $_"
    exit 1
}
