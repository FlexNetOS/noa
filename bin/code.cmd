@echo off
REM VS Code containment launcher (Phase 19 T880)
SETLOCAL EnableDelayedExpansion

SET "NOA_ROOT=%~dp0.."
SET "NOA_DATA=%NOA_ROOT%\data"
SET "APP_ID=vscode"
SET "APP_DATA=%NOA_DATA%\apps\%APP_ID%"

IF NOT EXIST "%APP_DATA%" (
    mkdir "%APP_DATA%" 1>nul 2>nul
)

SET "VSCODE_USER_DATA_DIR=%APP_DATA%\\User"
SET "VSCODE_EXTENSIONS=%APP_DATA%\\extensions"
SET "APPDATA=%APP_DATA%"
SET "LOCALAPPDATA=%APP_DATA%"

SET "HTTP_PROXY=http://127.0.0.1:8085"
SET "HTTPS_PROXY=%HTTP_PROXY%"

SET "CANDIDATE=%NOA_ROOT%\\opt\\apps\\vscode\\Code.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

SET "CANDIDATE=%ProgramFiles%\\Microsoft VS Code\\Code.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

ECHO [WARN] VS Code binary not found.
ECHO Checked:
ECHO   - %NOA_ROOT%\opt\apps\vscode\Code.exe
ECHO   - %ProgramFiles%\Microsoft VS Code\Code.exe
ECHO Install VS Code into opt\apps\vscode or update this wrapper.
EXIT /B 1
