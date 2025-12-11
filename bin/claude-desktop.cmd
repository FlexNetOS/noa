@echo off
REM Claude Desktop launcher with NOA containment (Phase 19 T863)
SETLOCAL EnableDelayedExpansion

SET "NOA_ROOT=%~dp0.."
SET "NOA_DATA=%NOA_ROOT%\data"
SET "APP_ID=claude"
SET "APP_DATA=%NOA_DATA%\apps\%APP_ID%"

IF NOT EXIST "%APP_DATA%" (
    mkdir "%APP_DATA%" 1>nul 2>nul
)

SET "APPDATA=%APP_DATA%"
SET "LOCALAPPDATA=%APP_DATA%"

SET "HTTP_PROXY=http://127.0.0.1:8085"
SET "HTTPS_PROXY=%HTTP_PROXY%"

SET "CANDIDATE=%NOA_ROOT%\opt\apps\claude\Claude.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

SET "CANDIDATE=%ProgramFiles%\\Claude\\Claude.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

ECHO [WARN] Claude Desktop binary not found.
ECHO Expected at:
ECHO   - %NOA_ROOT%\opt\apps\claude\Claude.exe
ECHO   - %ProgramFiles%\Claude\Claude.exe
ECHO Install Claude Desktop into opt\apps\claude or update this wrapper.
EXIT /B 1
