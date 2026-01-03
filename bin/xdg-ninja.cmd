@echo off
REM NOA xdg-ninja wrapper - XDG compliance audit
REM Constitution §3.1 compliant: audit for policy violations

set NOA_ROOT=N:\noa
set XDG_CONFIG_HOME=%NOA_ROOT%\etc
set XDG_CACHE_HOME=%NOA_ROOT%\cache
set XDG_DATA_HOME=%NOA_ROOT%\data

powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%NOA_ROOT%\scripts\xdg-ninja.ps1" %*
