param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^\d+$')]
    [string]$ProblemNumber
)

$repoRoot = Split-Path -Parent $PSScriptRoot
$template = Join-Path $repoRoot "src/bin/probTemplate.rs"
$formattedNumber = "{0:d4}" -f [int]$ProblemNumber
$target = Join-Path $repoRoot "src/bin/prob$formattedNumber.rs"

Copy-Item -Path $template -Destination $target -Force
Write-Host "Created $target"
