@echo off
REM Constitution §3.1 - NOA-hosted node wrapper
REM Delegates to opt\node\node.exe with proper NOA_ROOT resolution

setlocal
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%..") do set "NOA_ROOT=%%~fI"
set "PATH=%NOA_ROOT%\bin;%NOA_ROOT%\opt\node;%PATH%"

"%NOA_ROOT%\opt\node\node.exe" %*
