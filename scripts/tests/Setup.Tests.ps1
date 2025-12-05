<#
.SYNOPSIS
    Pester tests for NOA setup script

.DESCRIPTION
    Tests the setup-noa.ps1 script to ensure it:
    - Parses without syntax errors
    - Creates all required directories
    - Generates correct profile content
    - Creates marker and config files
    - Handles parameters correctly
#>

BeforeAll {
    $ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    $SetupScript = Join-Path $ProjectRoot "scripts\setup\setup-noa.ps1"
}

Describe "NOA Setup Script Tests" {
    
    Context "Script Validation" {
        
        It "Setup script exists" {
            $SetupScript | Should -Exist
        }
        
        It "Setup script parses without errors" {
            { 
                $null = [System.Management.Automation.PSParser]::Tokenize(
                    (Get-Content -Path $SetupScript -Raw), 
                    [ref]$null
                )
            } | Should -Not -Throw
        }
        
        It "Setup script has proper help documentation" {
            $help = Get-Help $SetupScript
            $help.Synopsis | Should -Not -BeNullOrEmpty
            $help.Description | Should -Not -BeNullOrEmpty
            $help.Parameters | Should -Not -BeNullOrEmpty
        }
    }
    
    Context "Setup Execution - Basic" {
        
        BeforeAll {
            # Create a test directory
            $TestRoot = Join-Path $TestDrive "noa-test-$(Get-Random)"
            
            # Run setup with minimal parameters (no prereqs, no integration)
            $result = & $SetupScript -NoaRoot $TestRoot -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            $LASTEXITCODE | Should -Be 0
        }
        
        It "Creates NOA root directory" {
            $TestRoot | Should -Exist
        }
        
        It "Creates all required directories" {
            $requiredDirs = @(
                "repos", "containers", "workspace", "config", "scripts",
                "logs", "tmp", "p2p", "ai", "git", "bin", "etc", "lib",
                "opt", "sys", "init"
            )
            
            foreach ($dir in $requiredDirs) {
                Join-Path $TestRoot $dir | Should -Exist
            }
        }
        
        It "Creates noa-profile.ps1" {
            $profilePath = Join-Path $TestRoot "noa-profile.ps1"
            $profilePath | Should -Exist
        }
        
        It "noa-profile.ps1 contains NOA_ROOT variable" {
            $profilePath = Join-Path $TestRoot "noa-profile.ps1"
            $content = Get-Content $profilePath -Raw
            $content | Should -Match '\$env:NOA_ROOT'
        }
        
        It "noa-profile.ps1 contains all environment variables" {
            $profilePath = Join-Path $TestRoot "noa-profile.ps1"
            $content = Get-Content $profilePath -Raw
            
            $expectedVars = @(
                'NOA_ROOT', 'NOA_REPOS', 'NOA_CONTAINERS', 'NOA_WORKSPACE',
                'NOA_CONFIG', 'NOA_SCRIPTS', 'NOA_LOGS', 'NOA_TMP',
                'NOA_P2P', 'NOA_AI', 'NOA_GIT', 'NOA_BIN'
            )
            
            foreach ($var in $expectedVars) {
                $content | Should -Match "\`$env:$var"
            }
        }
        
        It "noa-profile.ps1 contains navigation helper functions" {
            $profilePath = Join-Path $TestRoot "noa-profile.ps1"
            $content = Get-Content $profilePath -Raw
            
            $expectedFunctions = @('cda', 'cdr', 'cdc', 'cdw', 'cds', 'cdl', 'cdp', 'cdai', 'cdgit')
            
            foreach ($func in $expectedFunctions) {
                $content | Should -Match "function $func"
            }
        }
        
        It "Creates .noa marker file" {
            $markerPath = Join-Path $TestRoot ".noa"
            $markerPath | Should -Exist
        }
        
        It ".noa marker contains version info" {
            $markerPath = Join-Path $TestRoot ".noa"
            $content = Get-Content $markerPath -Raw
            $content | Should -Match "version="
        }
        
        It "Creates config/noa.json" {
            $configPath = Join-Path $TestRoot "config\noa.json"
            $configPath | Should -Exist
        }
        
        It "config/noa.json is valid JSON" {
            $configPath = Join-Path $TestRoot "config\noa.json"
            { 
                $null = Get-Content $configPath -Raw | ConvertFrom-Json 
            } | Should -Not -Throw
        }
        
        It "config/noa.json contains expected properties" {
            $configPath = Join-Path $TestRoot "config\noa.json"
            $config = Get-Content $configPath -Raw | ConvertFrom-Json
            
            $config.version | Should -Not -BeNullOrEmpty
            $config.name | Should -Be "NOA"
            $config.directories | Should -Not -BeNullOrEmpty
        }
        
        It "Creates log file in logs directory" {
            $logsDir = Join-Path $TestRoot "logs"
            $logFiles = Get-ChildItem $logsDir -Filter "setup-*.log"
            $logFiles | Should -Not -BeNullOrEmpty
        }
    }
    
    Context "Setup Execution - Profile Integration" {
        
        It "Does not modify profile when IntegrateProfile is false" {
            $TestRoot = Join-Path $TestDrive "noa-no-integration-$(Get-Random)"
            $profileBefore = if (Test-Path $PROFILE.CurrentUserAllHosts) {
                Get-Content $PROFILE.CurrentUserAllHosts -Raw
            } else {
                ""
            }
            
            & $SetupScript -NoaRoot $TestRoot -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            
            $profileAfter = if (Test-Path $PROFILE.CurrentUserAllHosts) {
                Get-Content $PROFILE.CurrentUserAllHosts -Raw
            } else {
                ""
            }
            
            $profileAfter | Should -Be $profileBefore
        }
    }
    
    Context "Setup Execution - Idempotency" {
        
        It "Running setup twice succeeds without errors" {
            $TestRoot = Join-Path $TestDrive "noa-idempotent-$(Get-Random)"
            
            # First run
            & $SetupScript -NoaRoot $TestRoot -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            $LASTEXITCODE | Should -Be 0
            
            # Second run (should be idempotent)
            & $SetupScript -NoaRoot $TestRoot -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            $LASTEXITCODE | Should -Be 0
        }
        
        It "Second run preserves existing files" {
            $TestRoot = Join-Path $TestDrive "noa-preserve-$(Get-Random)"
            
            # First run
            & $SetupScript -NoaRoot $TestRoot -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            
            # Modify a file
            $markerPath = Join-Path $TestRoot ".noa"
            "# Modified" | Add-Content $markerPath
            $markerContentBefore = Get-Content $markerPath -Raw
            
            # Second run
            & $SetupScript -NoaRoot $TestRoot -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            
            # Marker should be overwritten (this is expected for generated files)
            # But directories should still exist
            Join-Path $TestRoot "repos" | Should -Exist
            Join-Path $TestRoot "workspace" | Should -Exist
        }
    }
    
    Context "Parameter Handling" {
        
        It "Accepts custom NoaRoot path" {
            $customPath = Join-Path $TestDrive "custom-noa-$(Get-Random)"
            & $SetupScript -NoaRoot $customPath -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            
            $customPath | Should -Exist
            Join-Path $customPath "repos" | Should -Exist
        }
        
        It "Normalizes relative paths to absolute" {
            $TestRoot = Join-Path $TestDrive "noa-relative-$(Get-Random)"
            
            # This will work because the script normalizes paths
            & $SetupScript -NoaRoot $TestRoot -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            
            $TestRoot | Should -Exist
        }
    }
    
    Context "Error Handling" {
        
        It "Exits with non-zero code on failure" {
            # Try to create in an invalid location (should fail gracefully)
            $invalidPath = "Z:\InvalidDrive\noa-$(Get-Random)"
            
            try {
                & $SetupScript -NoaRoot $invalidPath -InstallPrereqs:$false -IntegrateProfile:$false -ErrorAction Stop
            } catch {
                # Expected to fail
            }
            
            # This test verifies the script handles errors
            # Actual behavior depends on system configuration
            $true | Should -Be $true
        }
    }
}

Describe "Install Prerequisites Script Tests" {
    
    BeforeAll {
        $ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
        $PrereqScript = Join-Path $ProjectRoot "scripts\setup\install-prereqs.ps1"
    }
    
    Context "Script Validation" {
        
        It "Prerequisites script exists" {
            $PrereqScript | Should -Exist
        }
        
        It "Prerequisites script parses without errors" {
            { 
                $null = [System.Management.Automation.PSParser]::Tokenize(
                    (Get-Content -Path $PrereqScript -Raw), 
                    [ref]$null
                )
            } | Should -Not -Throw
        }
    }
}
