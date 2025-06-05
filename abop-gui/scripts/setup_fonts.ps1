# Create assets directory if it doesn't exist
$assetsDir = "$PSScriptRoot/../../assets/fonts"
if (-not (Test-Path -Path $assetsDir)) {
    New-Item -ItemType Directory -Path $assetsDir -Force | Out-Null
}

# Download Font Awesome 6 Free
$fontAwesomeUrl = "https://use.fontawesome.com/releases/v6.5.1/fontawesome-free-6.5.1-desktop.zip"
$tempZip = "$env:TEMP\fontawesome-free.zip"
$extractPath = "$env:TEMP\fontawesome-free"

Write-Host "Downloading Font Awesome..."
Invoke-WebRequest -Uri $fontAwesomeUrl -OutFile $tempZip

# Extract the zip file
Write-Host "Extracting Font Awesome..."
Expand-Archive -Path $tempZip -DestinationPath $extractPath -Force

# Copy the font files
$sourceFile = "$extractPath\fontawesome-free-6.5.1-desktop\otfs\Font Awesome 6 Free-Solid-900.otf"
$destFile = "$assetsDir\Font Awesome 6 Free-Solid-900.otf"

if (Test-Path $sourceFile) {
    Copy-Item -Path $sourceFile -Destination $destFile -Force
    Write-Host "Font Awesome fonts have been installed to $assetsDir"
} else {
    Write-Error "Failed to find Font Awesome font files in the downloaded package."
}

# Clean up
Remove-Item -Path $tempZip -Force
Remove-Item -Path $extractPath -Recurse -Force

Write-Host "Font setup complete!"
