param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$ProblemNumbers
)

if (-not $ProblemNumbers -or $ProblemNumbers.Count -eq 0) {
    Write-Error "Usage: .\new_prob.ps1 <problem_number> [<problem_number> ...]"
    exit 1
}

$repoRoot = Split-Path -Parent $PSScriptRoot
$template = Join-Path $repoRoot "src/bin/probTemplate.rs"


foreach ($numText in $ProblemNumbers) {
    if ($numText -notmatch '^\d+$') {
        Write-Warning "Skipping invalid input: $numText"
        continue
    }

    $formattedNumber = "{0:d4}" -f [int]$numText
    $target = Join-Path $repoRoot "src/bin/prob$formattedNumber.rs"

    Copy-Item -Path $template -Destination $target -Force
    Write-Host "Created $target"
}
