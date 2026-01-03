@echo off
REM Constitution §3.1 - NOA-hosted npm wrapper
REM Delegates to opt\node\npm.cmd with proper NOA_ROOT resolution

setlocal
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%..") do set "NOA_ROOT=%%~fI"
set "PATH=%NOA_ROOT%\bin;%NOA_ROOT%\opt\node;%PATH%"

call "%NOA_ROOT%\opt\node\npm.cmd" %*
