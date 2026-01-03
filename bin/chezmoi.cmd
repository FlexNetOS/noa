@echo off
REM NOA chezmoi wrapper - dotfile management
REM Constitution §3.1 compliant: all paths resolve under noa_root

set NOA_ROOT=N:\noa

REM chezmoi configuration (§3.1 compliant paths)
set CHEZMOI_SOURCE_DIR=%NOA_ROOT%\etc\dotfiles
set CHEZMOI_CONFIG=%NOA_ROOT%\etc\chezmoi\chezmoi.toml
set CHEZMOI_CACHE_DIR=%NOA_ROOT%\cache\chezmoi

REM XDG directories for tool consistency
set XDG_CONFIG_HOME=%NOA_ROOT%\etc
set XDG_CACHE_HOME=%NOA_ROOT%\cache
set XDG_DATA_HOME=%NOA_ROOT%\data

REM Execute chezmoi via pixi environment
"%NOA_ROOT%\bin\pixi.cmd" run chezmoi %*
