# ABOP Database Centralization Test Script
$ErrorActionPreference = "Stop"

Write-Host "🔍 ABOP Database Centralization Test" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

# Get the expected database path
function Get-ExpectedDatabasePath {
    $dataDir = [Environment]::GetFolderPath("ApplicationData")
    return Join-Path $dataDir "abop-iced\database.db"
}

# Check if database exists
function Test-DatabaseExists {
    param([string]$DbPath)
    
    Write-Host "🗄️  Testing database at: $DbPath" -ForegroundColor Yellow
    
    if (Test-Path $DbPath) {
        $fileInfo = Get-Item $DbPath
        Write-Host "✅ Database exists" -ForegroundColor Green
        Write-Host "   Size: $($fileInfo.Length) bytes" -ForegroundColor Gray
        Write-Host "   Modified: $($fileInfo.LastWriteTime)" -ForegroundColor Gray
        return $true
    } else {
        Write-Host "❌ Database does not exist" -ForegroundColor Red
        return $false
    }
}

# Create test library
function Create-TestLibrary {
    param([string]$Path)
    
    Write-Host "📁 Creating test library at: $Path" -ForegroundColor Yellow
    
    if (Test-Path $Path) {
        Remove-Item $Path -Recurse -Force
    }
    New-Item -ItemType Directory -Path $Path -Force | Out-Null
    
    $testFiles = @(
        "Book 1 - Author A.mp3",
        "Book 2 - Author B.m4a", 
        "Book 3 - Author C.flac"
    )
    
    foreach ($file in $testFiles) {
        $filePath = Join-Path $Path $file
        "Test audiobook content for $file" | Out-File -FilePath $filePath -Encoding UTF8
    }
    
    Write-Host "✅ Created test library with $($testFiles.Length) files" -ForegroundColor Green
}

# Run CLI command
function Invoke-AbopCli {
    param([string]$Arguments)
    
    Write-Host "🖥️  Running CLI: cargo run --bin abop-cli -- $Arguments" -ForegroundColor Blue
    
    try {
        Set-Location "c:\Users\pshap\coding\abop"
        $result = & cargo run --bin abop-cli -- $Arguments.Split(' ')
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ CLI command succeeded" -ForegroundColor Green
        } else {
            Write-Host "❌ CLI command failed with exit code $LASTEXITCODE" -ForegroundColor Red
        }
        
        return $LASTEXITCODE
    } catch {
        Write-Host "❌ Failed to run CLI command: $($_.Exception.Message)" -ForegroundColor Red
        return -1
    }
}

# Main test execution
try {
    Write-Host "`n📍 Step 1: Database Path Verification" -ForegroundColor Cyan
    $expectedDbPath = Get-ExpectedDatabasePath
    Write-Host "Expected database location: $expectedDbPath" -ForegroundColor White
    Test-DatabaseExists -DbPath $expectedDbPath

    Write-Host "`n📁 Step 2: Test Library Setup" -ForegroundColor Cyan
    $testLibraryPath = "C:\temp\test_abop_library"
    Create-TestLibrary -Path $testLibraryPath

    Write-Host "`n📊 Step 3: Testing CLI Database Operations" -ForegroundColor Cyan
    Write-Host "Testing library list..." -ForegroundColor Yellow
    Invoke-AbopCli "--database list-libraries"

    Write-Host "`nTesting database stats..." -ForegroundColor Yellow
    Invoke-AbopCli "--database stats"

    Write-Host "`n🔍 Step 4: Post-CLI Database State" -ForegroundColor Cyan
    Test-DatabaseExists -DbPath $expectedDbPath

    Write-Host "`n🧪 Step 5: Running Validation Tool" -ForegroundColor Cyan
    Set-Location "c:\Users\pshap\coding\abop"
    & cargo run --example validate_centralization
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Validation tool completed successfully" -ForegroundColor Green
    } else {
        Write-Host "❌ Validation tool failed" -ForegroundColor Red
    }

    Write-Host "`n🧹 Step 6: Cleanup" -ForegroundColor Cyan
    if (Test-Path $testLibraryPath) {
        Remove-Item $testLibraryPath -Recurse -Force
        Write-Host "✅ Cleaned up test library" -ForegroundColor Green
    }

    Write-Host "`n🎉 Database centralization test completed!" -ForegroundColor Green

} catch {
    Write-Host "`n❌ Test failed with error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
