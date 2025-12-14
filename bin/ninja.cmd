@echo off
REM NOA Ninja wrapper - Constitutional §3.1 compliance
REM Points to portable Ninja in opt/ninja/

setlocal
set "NOA_ROOT=%~dp0.."
set "NINJA_EXE=%NOA_ROOT%\opt\ninja\ninja.exe"

if exist "%NINJA_EXE%" (
    "%NINJA_EXE%" %*
) else (
    echo [ERROR] Ninja not found at %NINJA_EXE%
    echo Run: .\scripts\bootstrap\installers\ninja-portable.ps1
    exit /b 1
)
