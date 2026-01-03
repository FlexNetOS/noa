@echo off
REM Constitution §3.1 - NOA-hosted npx wrapper
REM Delegates to opt\node\npx.cmd with proper NOA_ROOT resolution

setlocal
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%..") do set "NOA_ROOT=%%~fI"
set "PATH=%NOA_ROOT%\bin;%NOA_ROOT%\opt\node;%PATH%"

call "%NOA_ROOT%\opt\node\npx.cmd" %*
