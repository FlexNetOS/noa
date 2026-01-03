@echo off
REM direnv wrapper - Directory-based environment manager
REM Configured to use NOA-compliant paths per Constitution §3.1

set "NOA_ROOT=N:\noa"
set "XDG_CONFIG_HOME=%NOA_ROOT%\etc"
set "XDG_CACHE_HOME=%NOA_ROOT%\cache"
set "XDG_DATA_HOME=%NOA_ROOT%\opt"

REM Execute direnv via pixi environment
"%NOA_ROOT%\.pixi\envs\default\bin\direnv.exe" %*
