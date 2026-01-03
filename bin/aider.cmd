@echo off
REM NOA aider wrapper - AI pair programming terminal
REM Constitution §3.1 compliant: all paths resolve under noa_root

set NOA_ROOT=N:\noa

REM aider configuration (§3.1 compliant paths)
set AIDER_HOME=%NOA_ROOT%\etc\aider
set AIDER_CACHE=%NOA_ROOT%\cache\aider

REM XDG directories for tool consistency
set XDG_CONFIG_HOME=%NOA_ROOT%\etc
set XDG_CACHE_HOME=%NOA_ROOT%\cache
set XDG_DATA_HOME=%NOA_ROOT%\data

REM Execute aider via pixi environment (installed via pixi add aider-chat)
"%NOA_ROOT%\bin\pixi.cmd" run aider %*
