@echo off
REM NOA 0install wrapper - Zero Install decentralized package manager
REM Constitution §3.1 compliant: all paths resolve under noa_root

set NOA_ROOT=N:\noa

REM 0install configuration (§3.1 compliant paths)
set ZEROINSTALL_HOME=%NOA_ROOT%\opt\0install
set ZEROINSTALL_CACHE=%NOA_ROOT%\cache\0install

REM XDG directories for tool consistency
set XDG_CONFIG_HOME=%NOA_ROOT%\etc
set XDG_CACHE_HOME=%NOA_ROOT%\cache
set XDG_DATA_HOME=%NOA_ROOT%\data

REM Execute 0install
"%ZEROINSTALL_HOME%\0install.exe" %*
