@echo off
REM NOA Pixi Environment Wrapper
REM Runs commands inside the pixi environment
REM Constitution §3.1: All tools resolve under noa_root

REM Get NOA_ROOT from script location
for %%i in ("%~dp0..") do set "NOA_ROOT=%%~fi"

REM === XDG Base Directories ===
set "XDG_CONFIG_HOME=%NOA_ROOT%\etc"
set "XDG_DATA_HOME=%NOA_ROOT%\data"
set "XDG_CACHE_HOME=%NOA_ROOT%\cache"
set "XDG_STATE_HOME=%NOA_ROOT%\data\state"

REM === Pixi / Rattler ===
set "PIXI_HOME=%NOA_ROOT%\opt\pixi"
set "RATTLER_CACHE_DIR=%NOA_ROOT%\cache\pixi"
set "RATTLER_AUTH_FILE=%NOA_ROOT%\etc\rattler\credentials.json"

REM === Rust ===
set "CARGO_HOME=%NOA_ROOT%\opt\rust\cargo"
set "RUSTUP_HOME=%NOA_ROOT%\opt\rust\rustup"

REM === Go ===
set "GOPATH=%NOA_ROOT%\opt\go"
set "GOMODCACHE=%NOA_ROOT%\cache\go\mod"

REM === Node/NPM/PNPM ===
set "NPM_CONFIG_CACHE=%NOA_ROOT%\cache\npm"
set "NPM_CONFIG_PREFIX=%NOA_ROOT%\opt\npm-global"
set "NPM_CONFIG_USERCONFIG=%NOA_ROOT%\etc\npmrc"
set "PNPM_HOME=%NOA_ROOT%\opt\pnpm"

REM === Python ===
set "PIP_CACHE_DIR=%NOA_ROOT%\cache\pip"
set "PYTHONUSERBASE=%NOA_ROOT%\opt\python"
set "CONDA_PREFIX=%NOA_ROOT%\opt\conda"

REM === Ollama ===
set "OLLAMA_HOME=%NOA_ROOT%\opt\ollama"
set "OLLAMA_MODELS=%NOA_ROOT%\cache\ollama\models"

REM === Docker ===
set "DOCKER_CONFIG=%NOA_ROOT%\etc\docker"

REM === AI Providers ===
set "CLAUDE_CONFIG_DIR=%NOA_ROOT%\etc\claude"

REM === Security ===
set "GNUPGHOME=%NOA_ROOT%\etc\gnupg"

REM === .NET / NuGet ===
set "NUGET_PACKAGES=%NOA_ROOT%\cache\nuget"
set "DOTNET_CLI_HOME=%NOA_ROOT%\opt\dotnet"

REM === Cloud CLI ===
set "AZURE_CONFIG_DIR=%NOA_ROOT%\etc\azure"
set "AWS_CONFIG_FILE=%NOA_ROOT%\etc\aws\config"
set "AWS_SHARED_CREDENTIALS_FILE=%NOA_ROOT%\etc\aws\credentials"

REM === Git ===
set "GIT_CONFIG_GLOBAL=%NOA_ROOT%\etc\gitconfig"

REM === Browser Automation ===
set "PLAYWRIGHT_BROWSERS_PATH=%NOA_ROOT%\cache\playwright"

REM === ML/AI Models ===
set "HUGGINGFACE_HUB_CACHE=%NOA_ROOT%\cache\huggingface"
set "HF_HOME=%NOA_ROOT%\cache\huggingface"

REM === PATH ===
set "PATH=%NOA_ROOT%\opt\pixi\bin;%NOA_ROOT%\.pixi\envs\default\bin;%NOA_ROOT%\.pixi\envs\default\Library\bin;%NOA_ROOT%\bin;%PATH%"

cd /d "%NOA_ROOT%"
"%NOA_ROOT%\opt\pixi\bin\pixi.exe" %*
