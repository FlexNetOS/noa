@echo off
REM NOA CMake wrapper - Constitutional §3.1 compliance
REM Points to portable CMake in opt/cmake/

setlocal
set "NOA_ROOT=%~dp0.."
set "CMAKE_EXE=%NOA_ROOT%\opt\cmake\bin\cmake.exe"

if exist "%CMAKE_EXE%" (
    "%CMAKE_EXE%" %*
) else (
    echo [ERROR] CMake not found at %CMAKE_EXE%
    echo Run: .\scripts\bootstrap\installers\cmake-portable.ps1
    exit /b 1
)

