# FAQ 搜索脚本 (PowerShell)
# 用法: .\faq_search.ps1 -Keyword <关键词>

param(
    [Parameter(Mandatory=$true)]
    [string]$Keyword
)

$FAQ_FILE = Join-Path $PSScriptRoot "FAQ.md"

if (-not (Test-Path $FAQ_FILE)) {
    Write-Error "错误: FAQ.md 文件不存在"
    exit 1
}

Write-Host "===== FAQ 搜索结果: $Keyword =====" -ForegroundColor Green
Write-Host ""

# 搜索内容
Select-String -Path $FAQ_FILE -Pattern $Keyword -Context 0,5 | ForEach-Object {
    $lineNum = $_.LineNumber
    $line = $_.Line
    $context = $_.Context.PostContext -join "`n"
    
    Write-Host "行 $lineNum:" -ForegroundColor Yellow
    Write-Host "$line" -ForegroundColor Cyan
    if ($context) {
        Write-Host "$context"
    }
    Write-Host "---"
}

Write-Host ""
Write-Host "查看完整 FAQ: docs/FAQ.md" -ForegroundColor Green