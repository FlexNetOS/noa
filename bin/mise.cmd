@echo off
REM mise wrapper - Unified tool/env/task manager
REM Configured to use NOA-compliant paths per Constitution §3.1

REM Set NOA root
set "NOA_ROOT=N:\noa"

REM mise-specific directories
set "MISE_DATA_DIR=%NOA_ROOT%\opt\mise"
set "MISE_CONFIG_DIR=%NOA_ROOT%\etc\mise"
set "MISE_CACHE_DIR=%NOA_ROOT%\cache\mise"
set "MISE_STATE_DIR=%NOA_ROOT%\data\state\mise"

REM XDG Base Directories (mise respects these too)
set "XDG_CONFIG_HOME=%NOA_ROOT%\etc"
set "XDG_CACHE_HOME=%NOA_ROOT%\cache"
set "XDG_DATA_HOME=%NOA_ROOT%\opt"
set "XDG_STATE_HOME=%NOA_ROOT%\data\state"

REM Execute mise
"%NOA_ROOT%\opt\mise\bin\mise.exe" %*
