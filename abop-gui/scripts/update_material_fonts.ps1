# Update Material Design fonts for ABOP GUI
# This script downloads all required Roboto font variants for Material Design 3

# Create assets directory if it doesn't exist
$fontsDir = "$PSScriptRoot/../assets/fonts/roboto"
if (-not (Test-Path -Path $fontsDir)) {
    New-Item -ItemType Directory -Path $fontsDir -Force | Out-Null
    Write-Host "Created directory: $fontsDir"
}

# Font URLs for Google's Roboto
$fontUrls = @{
    "Roboto-Regular.woff2" = "https://fonts.gstatic.com/s/roboto/v30/KFOmCnqEu92Fr1Mu4mxK.woff2"
    "Roboto-Medium.woff2" = "https://fonts.gstatic.com/s/roboto/v30/KFOlCnqEu92Fr1MmEU9fBBc4.woff2"
    "Roboto-Bold.woff2" = "https://fonts.gstatic.com/s/roboto/v30/KFOlCnqEu92Fr1MmWUlfBBc4.woff2"
}

foreach ($font in $fontUrls.GetEnumerator()) {
    $outFile = "$fontsDir/$($font.Key)"
    Write-Host "Downloading $($font.Key)..."
    
    try {
        Invoke-WebRequest -Uri $font.Value -OutFile $outFile
        Write-Host "Downloaded $($font.Key) to $outFile" -ForegroundColor Green
    }
    catch {
        Write-Host "Failed to download $($font.Key): $_" -ForegroundColor Red
    }
}

Write-Host "Material Design fonts update complete!" -ForegroundColor Cyan
