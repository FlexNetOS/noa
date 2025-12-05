<#
  Legacy NOA Setup Script - DEPRECATED
  
  This script has been replaced by the comprehensive setup script.
  Please use: .\scripts\setup\setup-noa.ps1
  
  This wrapper is provided for backward compatibility.
#>

param(
    [string]$NoaRoot = "N:\noa"
)

Write-Host ""
Write-Host "========================================" -ForegroundColor Yellow
Write-Host "  NOTICE: This script is deprecated" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow
Write-Host ""
Write-Host "This script has been replaced with a more comprehensive version." -ForegroundColor Cyan
Write-Host ""
Write-Host "New location: .\scripts\setup\setup-noa.ps1" -ForegroundColor Green
Write-Host ""
Write-Host "The new script provides:" -ForegroundColor Cyan
Write-Host "  • Prerequisite installation (-InstallPrereqs)" -ForegroundColor White
Write-Host "  • PowerShell profile integration (-IntegrateProfile)" -ForegroundColor White
Write-Host "  • Comprehensive logging" -ForegroundColor White
Write-Host "  • Better error handling" -ForegroundColor White
Write-Host "  • Full directory structure" -ForegroundColor White
Write-Host ""
Write-Host "Redirecting to new script..." -ForegroundColor Yellow
Write-Host ""

# Call the new setup script
& "$PSScriptRoot\scripts\setup\setup-noa.ps1" -NoaRoot $NoaRoot

exit $LASTEXITCODE
