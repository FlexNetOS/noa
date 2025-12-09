<#
.SYNOPSIS
    NOA SSH Service Manager for Windows

.DESCRIPTION
    Manages SSH server contained within NOA root.
    Windows equivalent of scripts/ssh-service (bash)
    Uses Windows OpenSSH or portable sshd

.PARAMETER Action
    Action to perform: start, stop, status

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\ssh-service.ps1 -Action start
    .\ssh-service.ps1 -Action status
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("start", "stop", "status")]
    [string]$Action,

    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$SSHD_CONFIG = Join-Path $NoaRoot "etc/ssh/sshd_config"
$SSHD_PID = Join-Path $NoaRoot "init/run/sshd.pid"
$SSH_HOST_KEY = Join-Path $NoaRoot "etc/ssh/ssh_host_rsa_key"

# Find sshd (Windows OpenSSH or portable)
$SSHD_BIN = $null
$portableSshd = Join-Path $NoaRoot "bin/sshd.exe"
if (Test-Path $portableSshd) {
    $SSHD_BIN = $portableSshd
} else {
    $systemSshd = Get-Command "sshd" -ErrorAction SilentlyContinue
    if ($systemSshd) {
        $SSHD_BIN = $systemSshd.Source
    }
}

switch ($Action) {
    "start" {
        Write-Host "Starting NOA SSH server..." -ForegroundColor Cyan

        if (-not $SSHD_BIN) {
            Write-Error "sshd not found. Install Windows OpenSSH or place sshd.exe in $NoaRoot/bin/"
        }

        # Create directories
        $sshDir = Join-Path $NoaRoot "etc/ssh"
        $runDir = Join-Path $NoaRoot "init/run"
        @($sshDir, $runDir) | ForEach-Object {
            if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ -Force | Out-Null }
        }

        # Generate host key if not exists
        if (-not (Test-Path $SSH_HOST_KEY)) {
            Write-Host "  Generating SSH host key..." -ForegroundColor Gray
            $sshKeygen = Get-Command "ssh-keygen" -ErrorAction SilentlyContinue
            if ($sshKeygen) {
                & ssh-keygen -t rsa -b 4096 -f $SSH_HOST_KEY -N '""' -q
            } else {
                Write-Warning "ssh-keygen not found. Host key not generated."
            }
        }

        # Create minimal sshd_config if not exists
        if (-not (Test-Path $SSHD_CONFIG)) {
            @"
# NOA SSH Server Configuration
Port 2222
HostKey $SSH_HOST_KEY
AuthorizedKeysFile $NoaRoot/etc/ssh/authorized_keys
PasswordAuthentication yes
PermitRootLogin no
Subsystem sftp sftp-server.exe
"@ | Set-Content -Path $SSHD_CONFIG
            Write-Host "  Created sshd_config (port 2222)" -ForegroundColor Gray
        }

        # Check if already running
        if (Test-Path $SSHD_PID) {
            $pid = Get-Content $SSHD_PID
            if (Get-Process -Id $pid -ErrorAction SilentlyContinue) {
                Write-Host "SSH is already running (PID $pid)" -ForegroundColor Yellow
                exit 0
            }
        }

        # Start sshd
        $process = Start-Process -FilePath $SSHD_BIN -ArgumentList "-f", $SSHD_CONFIG -PassThru -WindowStyle Hidden
        $process.Id | Set-Content -Path $SSHD_PID
        Write-Host "SSH server started (PID $($process.Id))" -ForegroundColor Green
    }

    "stop" {
        Write-Host "Stopping SSH server..." -ForegroundColor Cyan

        if (Test-Path $SSHD_PID) {
            $pid = Get-Content $SSHD_PID
            $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
            if ($process) {
                Stop-Process -Id $pid -Force
                Write-Host "SSH server stopped" -ForegroundColor Green
            }
            Remove-Item -Path $SSHD_PID -Force
        } else {
            Write-Host "SSH PID file not found" -ForegroundColor Yellow
        }
    }

    "status" {
        if (Test-Path $SSHD_PID) {
            $pid = Get-Content $SSHD_PID
            $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
            if ($process) {
                Write-Host "SSH is running (PID $pid)" -ForegroundColor Green
            } else {
                Write-Host "SSH is not running (stale PID file)" -ForegroundColor Yellow
            }
        } else {
            Write-Host "SSH is not running" -ForegroundColor Yellow
        }
    }
}

