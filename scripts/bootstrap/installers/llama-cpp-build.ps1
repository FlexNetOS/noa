<#
.SYNOPSIS
    Build llama.cpp from source in noa_root/opt/llama.cpp/

.DESCRIPTION
    Builds llama.cpp using CMake. Requires:
    - CMake 3.14+
    - Visual Studio Build Tools or MSVC
    - Optional: CUDA toolkit for GPU support

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from repo root)

.PARAMETER GpuLayers
    Enable CUDA GPU acceleration (requires CUDA toolkit)

.PARAMETER Clean
    Clean build (remove existing build directory)

.EXAMPLE
    .\llama-cpp-build.ps1
    .\llama-cpp-build.ps1 -GpuLayers
    .\llama-cpp-build.ps1 -Clean
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$GpuLayers,
    [switch]$Clean
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        # Walk up from script location to find repo root
        $scriptDir = $PSScriptRoot
        $current = $scriptDir
        while ($current -and -not (Test-Path (Join-Path $current ".git"))) {
            $current = Split-Path -Parent $current
        }
        if ($current) { $current } else { Get-Location }
    }
}

$LLAMA_DIR = Join-Path $NoaRoot "opt\llama.cpp"
$BUILD_DIR = Join-Path $LLAMA_DIR "build"
$BIN_DIR = Join-Path $NoaRoot "bin"

Write-Host "NOA llama.cpp Builder" -ForegroundColor Cyan
Write-Host "NOA_ROOT:   $NoaRoot" -ForegroundColor Gray
Write-Host "LLAMA_DIR:  $LLAMA_DIR" -ForegroundColor Gray
Write-Host "BUILD_DIR:  $BUILD_DIR" -ForegroundColor Gray
Write-Host ""

# Check prerequisites
Write-Host "[1/5] Checking prerequisites..." -ForegroundColor Yellow

# Check CMake
$cmake = Get-Command cmake -ErrorAction SilentlyContinue
if (-not $cmake) {
    Write-Host "  [ERROR] CMake not found. Install via:" -ForegroundColor Red
    Write-Host "    winget install Kitware.CMake" -ForegroundColor Gray
    exit 1
}
Write-Host "  [OK] CMake: $((cmake --version | Select-Object -First 1))" -ForegroundColor Green

# Check for Visual Studio / Build Tools
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsWhere) {
    $vsPath = & $vsWhere -latest -property installationPath 2>$null
    if ($vsPath) {
        Write-Host "  [OK] Visual Studio: $vsPath" -ForegroundColor Green
    }
} else {
    Write-Host "  [WARN] vswhere not found - CMake will try to find a compiler" -ForegroundColor Yellow
}

# Check CUDA if GPU requested
if ($GpuLayers) {
    # Check for CUDA in NOA portable location first
    $cudaPortable = Join-Path $NoaRoot "opt\cuda\toolkit"
    $nvccPortable = Join-Path $cudaPortable "bin\nvcc.exe"

    if (Test-Path $nvccPortable) {
        Write-Host "  [OK] CUDA (Portable): $cudaPortable" -ForegroundColor Green
        $env:CUDA_PATH = $cudaPortable
        $env:CUDA_HOME = $cudaPortable
        $env:PATH = "$cudaPortable\bin;$env:PATH"
    } else {
        # Fall back to system CUDA
        $nvcc = Get-Command nvcc -ErrorAction SilentlyContinue
        if ($nvcc) {
            Write-Host "  [OK] CUDA (System): $((nvcc --version | Select-String 'release' | Select-Object -First 1))" -ForegroundColor Green
        } else {
            Write-Host "  [WARN] CUDA not found - building without GPU support" -ForegroundColor Yellow
            Write-Host "  Install CUDA: powershell scripts/bootstrap/installers/cuda-portable.ps1" -ForegroundColor Gray
            $GpuLayers = $false
        }
    }
}

# Check llama.cpp submodule
Write-Host "[2/5] Checking llama.cpp submodule..." -ForegroundColor Yellow

if (-not (Test-Path $LLAMA_DIR)) {
    Write-Host "  [ERROR] llama.cpp not found at $LLAMA_DIR" -ForegroundColor Red
    Write-Host "  Initialize submodule:" -ForegroundColor Gray
    Write-Host "    git submodule update --init opt/llama.cpp" -ForegroundColor Gray
    exit 1
}

if (-not (Test-Path (Join-Path $LLAMA_DIR "CMakeLists.txt"))) {
    Write-Host "  [ERROR] CMakeLists.txt not found - submodule may be empty" -ForegroundColor Red
    Write-Host "  Reinitialize:" -ForegroundColor Gray
    Write-Host "    git submodule update --init --recursive opt/llama.cpp" -ForegroundColor Gray
    exit 1
}

Write-Host "  [OK] llama.cpp source found" -ForegroundColor Green

# Clean build if requested
if ($Clean -and (Test-Path $BUILD_DIR)) {
    Write-Host "[3/5] Cleaning previous build..." -ForegroundColor Yellow
    Remove-Item -Path $BUILD_DIR -Recurse -Force
    Write-Host "  [OK] Cleaned" -ForegroundColor Green
} else {
    Write-Host "[3/5] Using existing build directory (if any)" -ForegroundColor Yellow
}

# configsure with CMake
Write-Host "[4/5] configsuring with CMake..." -ForegroundColor Yellow

$cmakeArgs = @(
    "-B", $BUILD_DIR,
    "-S", $LLAMA_DIR,
    "-DCMAKE_BUILD_TYPE=Release"
)

if ($GpuLayers) {
    $cmakeArgs += "-DGGML_CUDA=ON"
    $cmakeArgs += "-DCMAKE_CUDA_ARCHITECTURES=native"
    Write-Host "  CUDA GPU support: ENABLED" -ForegroundColor Cyan
} else {
    Write-Host "  CUDA GPU support: disabled (CPU only)" -ForegroundColor Gray
}

# Disable CURL dependency (not needed for basic inference)
$cmakeArgs += "-DLLAMA_CURL=OFF"

# Build only CLI, not server (server has httplib linking issues with MinGW)
$cmakeArgs += "-DBUILD_SHARED_LIBS=OFF"

# Use MinGW with MinGW Makefiles generator
$mingwBin = Join-Path $NoaRoot "opt\mingw\bin"
if (Test-Path $mingwBin) {
    $env:PATH = "$mingwBin;$env:PATH"
    $cmakeArgs += "-G", "MinGW Makefiles"
    $cmakeArgs += "-DCMAKE_C_COMPILER=$mingwBin\gcc.exe"
    $cmakeArgs += "-DCMAKE_CXX_COMPILER=$mingwBin\g++.exe"
    Write-Host "  Compiler: MinGW-w64 GCC" -ForegroundColor Cyan
    Write-Host "  Build system: MinGW Makefiles" -ForegroundColor Cyan
} else {
    # Use Ninja if available for faster builds
    $ninjaExe = Join-Path $NoaRoot "opt\ninja\ninja.exe"
    if (Test-Path $ninjaExe) {
        $cmakeArgs += "-G", "Ninja"
        $cmakeArgs += "-DCMAKE_MAKE_PROGRAM=$ninjaExe"
        Write-Host "  Build system: Ninja (fast)" -ForegroundColor Cyan
    } else {
        Write-Host "  Build system: MSBuild (default)" -ForegroundColor Gray
    }
}

Push-Location $LLAMA_DIR
try {
    & cmake @cmakeArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  [ERROR] CMake configsuration failed" -ForegroundColor Red
        exit 1
    }
    Write-Host "  [OK] configsuration complete" -ForegroundColor Green
} finally {
    Pop-Location
}

# Build
Write-Host "[5/5] Building llama.cpp..." -ForegroundColor Yellow
Write-Host "  This may take several minutes..." -ForegroundColor Gray

Push-Location $LLAMA_DIR
try {
    & cmake --build $BUILD_DIR --configs Release --parallel
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  [ERROR] Build failed" -ForegroundColor Red
        exit 1
    }
    Write-Host "  [OK] Build complete" -ForegroundColor Green
} finally {
    Pop-Location
}

# Find and report built binaries
Write-Host ""
Write-Host "=== Build Summary ===" -ForegroundColor Cyan

$releaseDir = Join-Path $BUILD_DIR "bin\Release"
$binSearchDir = if (Test-Path $releaseDir) { $releaseDir } else { Join-Path $BUILD_DIR "bin" }

if (Test-Path $binSearchDir) {
    Write-Host "Built binaries in ${binSearchDir}:" -ForegroundColor Green

    $binaries = Get-ChildItem $binSearchDir -Filter "*.exe" -ErrorAction SilentlyContinue
    foreach ($bin in $binaries) {
        Write-Host "  - $($bin.Name)" -ForegroundColor Gray
    }

    # Check for key binaries
    $llamaServer = Join-Path $binSearchDir "llama-server.exe"
    $llamaCli = Join-Path $binSearchDir "llama-cli.exe"

    if (Test-Path $llamaServer) {
        Write-Host ""
        Write-Host "[OK] llama-server.exe ready" -ForegroundColor Green
        Write-Host "  Wrapper: $BIN_DIR\llama-server.cmd" -ForegroundColor Gray
    }

    if (Test-Path $llamaCli) {
        Write-Host "[OK] llama-cli.exe ready" -ForegroundColor Green
        Write-Host "  Wrapper: $BIN_DIR\llama-cli.cmd" -ForegroundColor Gray
    }
} else {
    Write-Host "[WARN] No binaries found in expected location" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "llama.cpp build complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  llama-server -m model.gguf --port 8080   # Start inference server" -ForegroundColor Gray
Write-Host "  llama-cli -m model.gguf -p 'Hello'       # Run inference" -ForegroundColor Gray
Write-Host ""
Write-Host "Download models from: https://huggingface.co/models?other=gguf" -ForegroundColor Yellow

