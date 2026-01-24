#!/usr/bin/env pwsh
<#
Lists the top longest Rust (.rs) files in the repo by line count.
Usage examples:
    pwsh -File scripts/metrics_long_rust_files.ps1
    pwsh -File scripts/metrics_long_rust_files.ps1 -Top 20 -Threshold 300
    pwsh -File scripts/metrics_long_rust_files.ps1 -Top 1 -Json
#>

param(
    [int]$Top = 10,
    [int]$Threshold = 300,
    [switch]$Json
)

$ErrorActionPreference = 'Stop'

# Exclude common generated or irrelevant directories
$excludeDirs = @(
    '\\target\\',
    '\\.git\\',
    '\\coverage\\',
    '\\material-web-clean\\',
    '\\assets\\fonts\\',
    '\\target-release\\',
    '\\target-debug\\'
)

function Test-ExcludedPath([string]$path) {
    foreach ($pattern in $excludeDirs) {
        if ($path -match $pattern) { return $true }
    }
    return $false
}

# Collect all .rs files and measure line counts
$results = @()
$files = Get-ChildItem -Recurse -File -Filter *.rs
foreach ($f in $files) {
    if (Test-ExcludedPath -path $f.FullName) { continue }
    $lines = (Get-Content -Path $f.FullName -Encoding UTF8 | Measure-Object -Line).Lines
    $results += [PSCustomObject]@{
        Lines   = $lines
        Path    = $f.FullName
        RelPath = (Resolve-Path -Path $f.FullName -Relative)
        Over    = [int]([Math]::Max(0, $lines - $Threshold))
    }
}

$sorted = $results | Sort-Object -Property Lines -Descending
$topN = $sorted | Select-Object -First $Top

if ($Json) {
    $topN | ConvertTo-Json -Depth 4 -Compress | Write-Output
    exit 0
}

Write-Host "\n🔎 Top $Top longest Rust files (threshold: $Threshold lines):" -ForegroundColor Cyan
foreach ($item in $topN) {
    $flag = if ($item.Lines -gt $Threshold) { '⚠️  exceeds' } else { '✅ within' }
    Write-Host ("{0,6} lines  {1} ({2} by {3} lines)" -f $item.Lines, $item.RelPath, $flag, $item.Over) -ForegroundColor White
}

$overCount = ($sorted | Where-Object { $_.Lines -gt $Threshold }).Count
if ($overCount -gt 0) {
    Write-Host ("\n⚠️  {0} files exceed {1} lines. Consider splitting modules or extracting components." -f $overCount, $Threshold) -ForegroundColor Yellow
} else {
    Write-Host ("\n✅ All files are within the {0}-line guideline." -f $Threshold) -ForegroundColor Green
}
