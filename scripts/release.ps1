# World Rules 发布脚本 (PowerShell)
# 用于创建版本 tag 并触发自动发布到 crates.io

param(
    [switch]$SkipVerify
)

# 错误时停止
$ErrorActionPreference = "Stop"

# 颜色函数
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

# 获取当前版本
$versionLine = Select-String -Path Cargo.toml -Pattern '^version = "([^"]+)"' | Select-Object -First 1
$version = $versionLine.Matches.Groups[1].Value

Write-ColorOutput Green "=== World Rules 发布脚本 ==="
Write-Output "当前版本: $version"
Write-Output ""

# 检查工作目录状态
$status = git status --porcelain
if ($status) {
    Write-ColorOutput Red "错误: 工作目录有未提交的更改"
    Write-Output "请先提交或暂存所有更改"
    git status
    exit 1
}

# 检查是否有未推送的提交
$local = git rev-parse HEAD
$remote = git rev-parse origin/master 2>$null
if ($local -ne $remote) {
    Write-ColorOutput Yellow "警告: 本地有未推送的提交"
    $answer = Read-Host "是否要先推送? (y/n)"
    if ($answer -eq "y") {
        git push origin master
    } else {
        Write-Output "取消发布"
        exit 1
    }
}

# 检查 tag 是否已存在
$tag = "v$version"
$tagExists = git rev-parse $tag 2>$null
if ($tagExists) {
    Write-ColorOutput Red "错误: Tag $tag 已存在"
    Write-Output "请更新 Cargo.toml 中的版本号"
    exit 1
}

# 运行验证
if (-not $SkipVerify) {
    Write-ColorOutput Yellow "运行验证检查..."
    
    Write-Output "1. 检查代码格式..."
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput Red "格式检查失败"
        exit 1
    }
    
    Write-Output "2. 运行 clippy..."
    cargo clippy -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput Red "Clippy 检查失败"
        exit 1
    }
    
    Write-Output "3. 检查发布包..."
    cargo publish --dry-run
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput Red "发布包验证失败"
        exit 1
    }
    
    Write-Output ""
    Write-ColorOutput Green "验证通过!"
}

Write-Output ""
Write-Output "发布说明:"
Write-Output "  - 版本: $tag"
Write-Output "  - 将发布到 crates.io"
Write-Output "  - 将创建 GitHub Release"
Write-Output ""
Write-ColorOutput Yellow "重要提示:"
Write-Output "  请确保已在 GitHub 设置中配置 CRATES_IO_TOKEN secret"
Write-Output "  https://github.com/hufengxiao/world-rules/settings/secrets/actions"
Write-Output ""

$continue = Read-Host "是否继续创建 tag 并推送? (y/n)"
if ($continue -eq "y") {
    Write-ColorOutput Yellow "创建 tag $tag..."
    git tag $tag
    
    Write-ColorOutput Yellow "推送 tag 到远程仓库..."
    git push origin $tag
    
    Write-Output ""
    Write-ColorOutput Green "✅ 发布已触发!"
    Write-Output "  - GitHub Actions 将自动发布到 crates.io"
    Write-Output "  - 查看进度: https://github.com/hufengxiao/world-rules/actions"
    Write-Output "  - 发布后查看: https://crates.io/crates/world_rules"
} else {
    Write-Output "取消发布"
}