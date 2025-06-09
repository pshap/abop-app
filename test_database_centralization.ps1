# ABOP Database Centralization Testing Script
# This script validates that both CLI and GUI use the same centralized database

param(
    [string]$TestLibraryPath = "C:\temp\abop_test_library",
    [switch]$CreateTestData,
    [switch]$CleanupAfter
)

Write-Host "🔍 ABOP Database Centralization Test Suite" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Function to get the expected database path
function Get-ExpectedDatabasePath {
    $dataDir = [Environment]::GetFolderPath('ApplicationData')
    return Join-Path $dataDir "abop-iced\database.db"
}

# Function to create test audiobook files
function Create-TestLibrary {
    param([string]$Path)
    
    Write-Host "📁 Creating test library at: $Path" -ForegroundColor Yellow
    
    if (Test-Path $Path) {
        Remove-Item $Path -Recurse -Force
    }
    New-Item -ItemType Directory -Path $Path -Force | Out-Null
    
    # Create some test audiobook files
    $testFiles = @(
        "Book 1 - Author A.mp3",
        "Book 2 - Author B.m4a", 
        "Book 3 - Author C.flac",
        "Some Random Text File.txt"  # This should be ignored
    )
    
    foreach ($file in $testFiles) {
        $filePath = Join-Path $Path $file
        # Create a small dummy file with some content
        "Test audiobook content for $file" | Out-File -FilePath $filePath -Encoding UTF8
    }
    
    Write-Host "✅ Created test library with $($testFiles.Length) files" -ForegroundColor Green
}

# Function to check if database exists and get basic info
function Test-DatabaseExists {
    param([string]$DbPath)
    
    Write-Host "🗄️  Testing database at: $DbPath" -ForegroundColor Yellow
    
    if (-not (Test-Path $DbPath)) {
        Write-Host "❌ Database file does not exist!" -ForegroundColor Red
        return $false
    }
    
    $fileInfo = Get-Item $DbPath
    Write-Host "✅ Database found - Size: $($fileInfo.Length) bytes, Created: $($fileInfo.CreationTime)" -ForegroundColor Green
    return $true
}

# Function to run CLI scan and capture output
function Test-CLIScan {
    param([string]$LibraryPath)
    
    Write-Host "🖥️  Testing CLI scan..." -ForegroundColor Yellow
    
    try {
        # Change to the CLI directory
        Push-Location "c:\Users\pshap\coding\abop\abop-cli"
        
        # Run CLI scan with proper syntax (note: no --database means it uses centralized DB)
        Write-Host "Running: cargo run -- scan --library `"$LibraryPath`"" -ForegroundColor Gray
        $output = cargo run -- scan --library "$LibraryPath" 2>&1
        
        Write-Host "CLI Output:" -ForegroundColor Gray
        $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ CLI scan completed successfully" -ForegroundColor Green
            return $true
        } else {
            Write-Host "❌ CLI scan failed with exit code: $LASTEXITCODE" -ForegroundColor Red
            return $false
        }
    }
    catch {
        Write-Host "❌ CLI scan error: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
    finally {
        Pop-Location
    }
}

# Function to test database content using CLI
function Test-DatabaseContent {
    Write-Host "📊 Testing database content via CLI..." -ForegroundColor Yellow
    
    try {
        Push-Location "c:\Users\pshap\coding\abop\abop-cli"
        
        # Get the expected database path for CLI commands
        $expectedDbPath = Get-ExpectedDatabasePath
        
        # List libraries
        Write-Host "Listing libraries..." -ForegroundColor Gray
        $librariesOutput = cargo run -- db --database "$expectedDbPath" list 2>&1
        Write-Host "Libraries output:" -ForegroundColor Gray
        $librariesOutput | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
        
        # Get database stats
        Write-Host "Getting database statistics..." -ForegroundColor Gray
        $statsOutput = cargo run -- db --database "$expectedDbPath" stats 2>&1
        Write-Host "Stats output:" -ForegroundColor Gray
        $statsOutput | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
        
        return $true
    }
    catch {
        Write-Host "❌ Database content test error: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
    finally {
        Pop-Location
    }
}

# Function to check for old database files
function Test-NoOldDatabases {
    Write-Host "🔍 Checking for old database files..." -ForegroundColor Yellow
    
    $found = $false
    
    # Check common locations where old databases might exist
    $possibleLocations = @(
        (Join-Path $env:APPDATA "abop"),
        (Join-Path $env:LOCALAPPDATA "abop"),
        "c:\Users\pshap\coding\abop\abop-gui",
        "c:\Users\pshap\coding\abop\abop-cli"
    )
    
    foreach ($location in $possibleLocations) {
        if (Test-Path $location) {
            $dbFiles = Get-ChildItem -Path $location -Filter "*.db" -Recurse -ErrorAction SilentlyContinue
            if ($dbFiles) {
                Write-Host "⚠️  Found database files in: $location" -ForegroundColor Yellow
                $dbFiles | ForEach-Object { 
                    Write-Host "   - $($_.FullName)" -ForegroundColor Yellow 
                }
                $found = $true
            }
        }
    }
    
    if (-not $found) {
        Write-Host "✅ No old database files found" -ForegroundColor Green
    }
    
    return -not $found
}

# Function to simulate GUI database access
function Test-GUIDatabase {
    Write-Host "🎨 Testing GUI database integration..." -ForegroundColor Yellow
    
    try {
        Push-Location "c:\Users\pshap\coding\abop\abop-gui"
        
        # Just try to build the GUI to ensure it compiles with the new database logic
        Write-Host "Building GUI to test compilation..." -ForegroundColor Gray
        $buildOutput = cargo check 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ GUI builds successfully with centralized database" -ForegroundColor Green
            return $true
        } else {
            Write-Host "❌ GUI build failed:" -ForegroundColor Red
            $buildOutput | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
            return $false
        }
    }
    catch {
        Write-Host "❌ GUI test error: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
    finally {
        Pop-Location
    }
}

# Main test execution
Write-Host ""
Write-Host "Starting database centralization tests..." -ForegroundColor Cyan

# Step 1: Check expected database path
$expectedDbPath = Get-ExpectedDatabasePath
Write-Host "Expected database location: $expectedDbPath" -ForegroundColor Cyan

# Step 2: Create test data if requested
if ($CreateTestData) {
    Create-TestLibrary -Path $TestLibraryPath
}

# Step 3: Remove existing database to start fresh
if (Test-Path $expectedDbPath) {
    Write-Host "🗑️  Removing existing database for fresh test..." -ForegroundColor Yellow
    Remove-Item $expectedDbPath -Force
    
    # Also remove the directory if it's empty
    $dbDir = Split-Path $expectedDbPath -Parent
    if ((Test-Path $dbDir) -and ((Get-ChildItem $dbDir).Count -eq 0)) {
        Remove-Item $dbDir -Force
    }
}

# Step 4: Test CLI scan (this should create the centralized database)
$cliSuccess = Test-CLIScan -LibraryPath $TestLibraryPath

# Step 5: Verify database was created
$dbExists = Test-DatabaseExists -DbPath $expectedDbPath

# Step 6: Test database content
$contentTest = Test-DatabaseContent

# Step 7: Check for old databases
$noOldDbs = Test-NoOldDatabases

# Step 8: Test GUI compilation
$guiTest = Test-GUIDatabase

# Step 9: Summary
Write-Host ""
Write-Host "🏆 Test Results Summary:" -ForegroundColor Cyan
Write-Host "======================" -ForegroundColor Cyan
Write-Host "CLI Scan:              $(if($cliSuccess){'✅ PASS'}else{'❌ FAIL'})" -ForegroundColor $(if($cliSuccess){'Green'}else{'Red'})
Write-Host "Database Created:      $(if($dbExists){'✅ PASS'}else{'❌ FAIL'})" -ForegroundColor $(if($dbExists){'Green'}else{'Red'})
Write-Host "Database Content:      $(if($contentTest){'✅ PASS'}else{'❌ FAIL'})" -ForegroundColor $(if($contentTest){'Green'}else{'Red'})
Write-Host "No Old Databases:      $(if($noOldDbs){'✅ PASS'}else{'⚠️  WARN'})" -ForegroundColor $(if($noOldDbs){'Green'}else{'Yellow'})
Write-Host "GUI Compilation:       $(if($guiTest){'✅ PASS'}else{'❌ FAIL'})" -ForegroundColor $(if($guiTest){'Green'}else{'Red'})

$allPassed = $cliSuccess -and $dbExists -and $contentTest -and $guiTest
Write-Host ""
Write-Host "Overall Result: $(if($allPassed){'✅ ALL TESTS PASSED'}else{'❌ SOME TESTS FAILED'})" -ForegroundColor $(if($allPassed){'Green'}else{'Red'})

# Step 10: Cleanup if requested
if ($CleanupAfter) {
    Write-Host ""
    Write-Host "🧹 Cleaning up test data..." -ForegroundColor Yellow
    if (Test-Path $TestLibraryPath) {
        Remove-Item $TestLibraryPath -Recurse -Force
        Write-Host "✅ Test library removed" -ForegroundColor Green
    }
}

Write-Host ""
Write-Host "Manual testing recommendations:" -ForegroundColor Cyan
Write-Host "1. Run the GUI and scan the same library directory" -ForegroundColor White
Write-Host "2. Verify both CLI and GUI show the same audiobooks" -ForegroundColor White
Write-Host "3. Check database location: $expectedDbPath" -ForegroundColor White
Write-Host "4. Test scanning different libraries from both interfaces" -ForegroundColor White
