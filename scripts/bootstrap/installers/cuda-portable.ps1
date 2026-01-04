param(
    [string]$NoaRoot = $env:NOA_ROOT,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

if (-not $NoaRoot) {
    $NoaRoot = Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))
}

$CudaInstaller = Join-Path $NoaRoot "opt\cuda\cuda_13.1.0_windows.exe"
$CudaTarget = Join-Path $NoaRoot "opt\cuda\toolkit"
$CudaTempExtract = Join-Path $NoaRoot "tmp\cuda-extract"

function Write-Status {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $color = switch ($Level) {
        "ERROR" { "Red" }
        "SUCCESS" { "Green" }
        "WARN" { "Yellow" }
        default { "Cyan" }
    }
    Write-Host "[$timestamp] [$Level] $Message" -ForegroundColor $color
}

Write-Status "CUDA Portable Installer" "INFO"
Write-Status "NOA_ROOT: $NoaRoot" "INFO"

if (-not (Test-Path $CudaInstaller)) {
    Write-Status "CUDA installer not found at: $CudaInstaller" "ERROR"
    exit 1
}

if ((Test-Path $CudaTarget) -and -not $Force) {
    Write-Status "CUDA already installed at: $CudaTarget" "SUCCESS"
    Write-Status "Use -Force to reinstall" "INFO"
    exit 0
}

Write-Status "Creating directories..." "INFO"
New-Item -ItemType Directory -Force -Path $CudaTarget | Out-Null
New-Item -ItemType Directory -Force -Path $CudaTempExtract | Out-Null

Write-Status "Extracting CUDA installer (this may take several minutes)..." "INFO"

try {
    # Try using 7-Zip if available
    $sevenZip = Get-Command "7z.exe" -ErrorAction SilentlyContinue

    if ($sevenZip) {
        Write-Status "Using 7-Zip for extraction..." "INFO"
        & 7z.exe x "$CudaInstaller" -o"$CudaTempExtract" -y | Out-Null
    } else {
        # Alternative: Use PowerShell's built-in extraction for self-extracting archives
        Write-Status "Attempting direct extraction..." "INFO"

        # CUDA installer is a self-extracting archive, we can extract it manually
        # by treating it as a 7z archive
        Add-Type -AssemblyName System.IO.Compression.FileSystem

        # Try to extract using .NET
        try {
            [System.IO.Compression.ZipFile]::ExtractToDirectory($CudaInstaller, $CudaTempExtract)
        } catch {
            Write-Status "Standard extraction failed. Trying alternative method..." "WARN"

            # Last resort: copy the installer and try to run it with extraction only
            $extractArgs = "-s -extract:`"$CudaTempExtract`""
            Start-Process -FilePath $CudaInstaller -ArgumentList $extractArgs -Wait -NoNewWindow -ErrorAction Stop
        }
    }

    Write-Status "Extraction complete, copying components..." "INFO"

    # Find and copy CUDA components
    $cudaComponents = Get-ChildItem -Path $CudaTempExtract -Recurse -Directory | Where-Object {
        $_.Name -match "cuda_|cudart|cublas|nvcc|nvrtc|curand|cusparse|cusolver|cufft|thrust"
    }

    if ($cudaComponents.Count -eq 0) {
        Write-Status "No CUDA components found. Copying entire extraction..." "WARN"
        Copy-Item -Path "$CudaTempExtract\*" -Destination $CudaTarget -Recurse -Force
    } else {
        foreach ($component in $cudaComponents) {
            Write-Status "Installing component: $($component.Name)" "INFO"
            Copy-Item -Path $component.FullName -Destination $CudaTarget -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Status "Setting up environment configsuration..." "INFO"

    $cudaEnvconfigs = @{
        CUDA_PATH = $CudaTarget
        CUDA_HOME = $CudaTarget
        CUDA_VERSION = "13.1"
        CUDA_PATH_V13_1 = $CudaTarget
    }

    $configsPath = Join-Path $NoaRoot "configs\cuda-env.json"
    $cudaEnvconfigs | ConvertTo-Json | Set-Content -Path $configsPath
    
    Write-Status "Cleaning up temporary files..." "INFO"
    Remove-Item -Path $CudaTempExtract -Recurse -Force -ErrorAction SilentlyContinue
    
    Write-Status "CUDA 13.1.0 installed successfully to: $CudaTarget" "SUCCESS"
    Write-Status "Add to PATH: $CudaTarget\bin" "INFO"
    Write-Status "Add to PATH: $CudaTarget\libnvvp" "INFO"
    
} catch {
    Write-Status "Installation failed: $_" "ERROR"
    Remove-Item -Path $CudaTempExtract -Recurse -Force -ErrorAction SilentlyContinue
    exit 1
}

$env:CUDA_PATH = $CudaTarget
$env:CUDA_HOME = $CudaTarget
$env:PATH = "$CudaTarget\bin;$CudaTarget\libnvvp;$env:PATH"

Write-Status "CUDA environment configsured for current session" "SUCCESS"
