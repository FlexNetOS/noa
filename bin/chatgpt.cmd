@echo off
REM ChatGPT Desktop launcher with NOA containment (Phase 19 T862)
SETLOCAL EnableDelayedExpansion

SET "NOA_ROOT=%~dp0.."
SET "NOA_DATA=%NOA_ROOT%\data"
SET "APP_ID=chatgpt"
SET "APP_DATA=%NOA_DATA%\apps\%APP_ID%"

IF NOT EXIST "%APP_DATA%" (
    mkdir "%APP_DATA%" 1>nul 2>nul
)

REM Redirect standard app data into NOA data volume
SET "APPDATA=%APP_DATA%"
SET "LOCALAPPDATA=%APP_DATA%"

REM Optional network proxy (aligned with config/desktop-apps.json)
SET "HTTP_PROXY=http://127.0.0.1:8085"
SET "HTTPS_PROXY=%HTTP_PROXY%"

REM Resolve executable location
SET "CANDIDATE=%NOA_ROOT%\opt\apps\chatgpt\ChatGPT.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

SET "CANDIDATE=%ProgramFiles%\\ChatGPT\\ChatGPT.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

ECHO [WARN] ChatGPT Desktop binary not found.
ECHO Looked in:
ECHO   - %NOA_ROOT%\opt\apps\chatgpt\ChatGPT.exe
ECHO   - %ProgramFiles%\ChatGPT\ChatGPT.exe
ECHO Install ChatGPT Desktop into opt\apps\chatgpt or adjust the wrapper.
EXIT /B 1
