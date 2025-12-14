@echo off
REM Cursor IDE containment launcher (Phase 19 T878)
SETLOCAL EnableDelayedExpansion

SET "NOA_ROOT=%~dp0.."
SET "NOA_DATA=%NOA_ROOT%\data"
SET "APP_ID=cursor"
SET "APP_DATA=%NOA_DATA%\apps\%APP_ID%"

IF NOT EXIST "%APP_DATA%" (
    mkdir "%APP_DATA%" 1>nul 2>nul
)

REM Enforce data redirection for Cursor (extensions + user data)
SET "CURSOR_USER_DATA_DIR=%APP_DATA%\\User"
SET "CURSOR_EXTENSIONS=%APP_DATA%\\extensions"
SET "APPDATA=%APP_DATA%"
SET "LOCALAPPDATA=%APP_DATA%"

SET "HTTP_PROXY=http://127.0.0.1:8085"
SET "HTTPS_PROXY=%HTTP_PROXY%"

SET "CANDIDATE=%NOA_ROOT%\\opt\\apps\\cursor\\Cursor.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

SET "CANDIDATE=%ProgramFiles%\\Cursor\\Cursor.exe"
IF EXIST "%CANDIDATE%" (
    "%CANDIDATE%" %*
    EXIT /B %ERRORLEVEL%
)

ECHO [WARN] Cursor binary not found.
ECHO Checked:
ECHO   - %NOA_ROOT%\opt\apps\cursor\Cursor.exe
ECHO   - %ProgramFiles%\Cursor\Cursor.exe
ECHO Install Cursor into opt\apps\cursor or update this wrapper.
EXIT /B 1
