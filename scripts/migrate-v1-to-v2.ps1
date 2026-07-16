# world-rules v1.x 到 v2.x 迁移脚本 (PowerShell)
# 用法: .\migrate-v1-to-v2.ps1 -ProjectPath "C:\path\to\your\project"

param(
    [string]$ProjectPath = ".",
    [switch]$DryRun = $false,
    [switch]$Verbose = $false
)

# 颜色输出函数
function Write-Info { param($msg) Write-Host $msg -ForegroundColor Cyan }
function Write-Success { param($msg) Write-Host "✅ $msg" -ForegroundColor Green }
function Write-Warning { param($msg) Write-Host "⚠️  $msg" -ForegroundColor Yellow }
function Write-Error { param($msg) Write-Host "❌ $msg" -ForegroundColor Red }

# 开始迁移
Write-Info "`n========================================"
Write-Info "world-rules v1.x → v2.x 迁移工具"
Write-Info "========================================`n"

# 检查项目路径
if (-not (Test-Path $ProjectPath)) {
    Write-Error "项目路径不存在: $ProjectPath"
    exit 1
}

$CargoFile = Join-Path $ProjectPath "Cargo.toml"
if (-not (Test-Path $CargoFile)) {
    Write-Error "未找到 Cargo.toml,请确认这是一个 Rust 项目"
    exit 1
}

Write-Info "项目路径: $ProjectPath"
Write-Info "Cargo 文件: $CargoFile`n"

# 读取 Cargo.toml
$Content = Get-Content $CargoFile -Raw
$OriginalContent = $Content

# 检查是否使用 world_rules
if ($Content -notmatch 'world_rules') {
    Write-Warning "Cargo.toml 中未发现 world_rules 依赖"
    Write-Info "如果您确定需要迁移,请手动添加依赖:"
    Write-Info '  [dependencies]'
    Write-Info '  world_rules = "2.0"'
    exit 0
}

# 显示当前依赖版本
Write-Info "当前依赖配置:"
if ($Content -match 'world_rules\s*=\s*"([^"]*)"') {
    $CurrentVersion = $matches[1]
    Write-Info "  版本: $CurrentVersion"
}

if ($Content -match 'world_rules\s*=\s*\{([^}]*)\}') {
    $DepsConfig = $matches[1]
    Write-Info "  配置: {$DepsConfig}"
}
Write-Host ""

# Dry run 模式
if ($DryRun) {
    Write-Warning "Dry run 模式 - 不会实际修改文件`n"
}

# 备份原文件
if (-not $DryRun) {
    $BackupFile = "$CargoFile.backup"
    Copy-Item $CargoFile $BackupFile
    Write-Success "已备份原文件到: $BackupFile"
}

# 更新版本
Write-Info "更新依赖版本..."

if ($Content -match 'world_rules\s*=\s*"[^"]*"') {
    # 简单版本格式: world_rules = "1.0"
    $Content = $Content -replace 'world_rules\s*=\s*"[^"]*"','world_rules = "2.0"'
    Write-Success "已更新简单版本格式"
}
elseif ($Content -match '(world_rules\s*=\s*\{[^}]*version\s*=\s*")[^"]*("\s*[^}]*\})') {
    # 复杂版本格式: world_rules = { version = "1.0", features = [...] }
    $Content = $Content -replace '(world_rules\s*=\s*\{[^}]*version\s*=\s*")[^"]*("\s*[^}]*\})', '${1}2.0${2}'
    Write-Success "已更新复杂版本格式"
}

# 写入文件
if (-not $DryRun) {
    Set-Content $CargoFile $Content -NoNewline
    Write-Success "已更新 Cargo.toml"
} else {
    Write-Info "预览更新内容:"
    Write-Host ($Content -replace '(?m)^', '  ')
}

# 更新 Cargo.lock
if (-not $DryRun) {
    Write-Info "`n更新 Cargo.lock..."
    Push-Location $ProjectPath
    
    $UpdateOutput = cargo update world_rules 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Success "已更新 Cargo.lock"
    } else {
        Write-Warning "Cargo.lock 更新失败: $UpdateOutput"
    }
    
    Pop-Location
}

# 检查编译
Write-Info "`n检查编译..."
Push-Location $ProjectPath

$CheckOutput = cargo check 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Success "编译检查通过"
} else {
    Write-Error "编译检查失败"
    if ($Verbose) {
        Write-Host $CheckOutput
    }
    Write-Warning "请手动检查编译错误"
}

Pop-Location

# 运行测试（可选）
Write-Info "`n检查测试..."
Push-Location $ProjectPath

$TestOutput = cargo test --no-run 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Success "测试编译通过"
} else {
    Write-Warning "测试编译失败"
    if ($Verbose) {
        Write-Host $TestOutput
    }
}

Pop-Location

# 总结
Write-Info "`n========================================"
Write-Info "迁移完成!"
Write-Info "========================================`n"

Write-Host "后续步骤:"
Write-Host "  1. 运行完整测试: cargo test"
Write-Host "  2. 检查代码质量: cargo clippy"
Write-Host "  3. 查看变更日志: docs/MIGRATION_GUIDE.md"
Write-Host "  4. 如有问题,恢复备份: $CargoFile.backup"
Write-Host ""

# 检查是否有新功能可用
Write-Info "v2.0 新功能提示:"
Write-Host "  - 性能检查系统: PerformanceChecker"
Write-Host "  - 规则难度分级: Difficulty"
Write-Host "  - 更多游戏和法律规则"
Write-Host "  - 性能提升约 20-25%"
Write-Host ""

if ($DryRun) {
    Write-Warning "这是 dry run,未实际修改文件"
    Write-Info "运行不带 -DryRun 参数来执行实际迁移"
}