@echo off
REM Portable Playwright CLI wrapper for NOA
REM Routes to project-local Playwright via pnpm
REM Usage: playwright.cmd [args...]

setlocal enabledelayedexpansion

REM Check if PLAYWRIGHT_BROWSERS_PATH is set for shared browser cache
if not defined PLAYWRIGHT_BROWSERS_PATH (
    set "PLAYWRIGHT_BROWSERS_PATH=%NOA_ROOT%\cache\playwright"
)

REM Priority: Check for project-local playwright first
if exist "package.json" (
    REM Use project-local playwright via pnpm
    pnpm exec playwright %*
    exit /b %ERRORLEVEL%
)

REM Fallback to known playwright locations
if exist "%NOA_ROOT%\sys\ui\apps\ml-devops\package.json" (
    pushd "%NOA_ROOT%\sys\ui\apps\ml-devops"
    pnpm exec playwright %*
    popd
    exit /b %ERRORLEVEL%
)

REM Last resort: npx
npx playwright %*
