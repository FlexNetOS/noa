@echo off
REM GitHub Desktop launcher with NOA containment (Phase 19 T864)
SETLOCAL EnableDelayedExpansion

SET "NOA_ROOT=%~dp0.."
SET "NOA_DATA=%NOA_ROOT%\data"
SET "APP_ID=github-desktop"
SET "APP_DATA=%NOA_DATA%\apps\%APP_ID%"

IF NOT EXIST "%APP_DATA%" (
    mkdir "%APP_DATA%" 1>nul 2>nul
)

REM Route GitHub Desktop data into NOA data volume
SET "APPDATA=%APP_DATA%"
SET "LOCALAPPDATA=%APP_DATA%"

SET "HTTP_PROXY=http://127.0.0.1:8085"
SET "HTTPS_PROXY=%HTTP_PROXY%"

REM Prefer NOA credential helper if available
SET "NOA_GIT_HELPER=%NOA_ROOT%\\bin\\git-credential-noa.cmd"
IF EXIST "%NOA_GIT_HELPER%" (
    SET "GIT_ASKPASS=%NOA_GIT_HELPER%"
)

SET "CANDIDATE=%NOA_ROOT%\opt\apps\github-desktop\GitHubDesktop.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

SET "CANDIDATE=%ProgramFiles%\\GitHub Desktop\\GitHubDesktop.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

ECHO [WARN] GitHub Desktop binary not found.
ECHO Checked:
ECHO   - %NOA_ROOT%\opt\apps\github-desktop\GitHubDesktop.exe
ECHO   - %ProgramFiles%\GitHub Desktop\GitHubDesktop.exe
ECHO Install GitHub Desktop into opt\apps\github-desktop or update this wrapper.
EXIT /B 1
