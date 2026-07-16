#!/bin/bash
# world-rules v1.x 到 v2.x 迁移脚本 (Bash)
# 用法: ./migrate-v1-to-v2.sh [project-path]

set -e

# 参数解析
PROJECT_PATH="${1:-.}"
DRY_RUN="${DRY_RUN:-false}"
VERBOSE="${VERBOSE:-false}"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

info() { echo -e "${CYAN}$1${NC}"; }
success() { echo -e "${GREEN}✅ $1${NC}"; }
warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
error() { echo -e "${RED}❌ $1${NC}"; }

# 开始迁移
info ""
info "========================================"
info "world-rules v1.x → v2.x 迁移工具"
info "========================================"
info ""

# 检查项目路径
if [ ! -d "$PROJECT_PATH" ]; then
    error "项目路径不存在: $PROJECT_PATH"
    exit 1
fi

CARGO_FILE="$PROJECT_PATH/Cargo.toml"
if [ ! -f "$CARGO_FILE" ]; then
    error "未找到 Cargo.toml,请确认这是一个 Rust 项目"
    exit 1
fi

info "项目路径: $PROJECT_PATH"
info "Cargo 文件: $CARGO_FILE"
info ""

# 读取 Cargo.toml
CONTENT=$(cat "$CARGO_FILE")

# 检查是否使用 world_rules
if ! echo "$CONTENT" | grep -q "world_rules"; then
    warning "Cargo.toml 中未发现 world_rules 依赖"
    info "如果您确定需要迁移,请手动添加依赖:"
    info "  [dependencies]"
    info '  world_rules = "2.0"'
    exit 0
fi

# 显示当前依赖版本
info "当前依赖配置:"
if echo "$CONTENT" | grep -oP 'world_rules\s*=\s*"\K[^"]+' > /dev/null 2>&1; then
    CURRENT_VERSION=$(echo "$CONTENT" | grep -oP 'world_rules\s*=\s*"\K[^"]+' | head -1)
    info "  版本: $CURRENT_VERSION"
fi
echo ""

# Dry run 模式
if [ "$DRY_RUN" = "true" ]; then
    warning "Dry run 模式 - 不会实际修改文件"
    echo ""
fi

# 备份原文件
if [ "$DRY_RUN" != "true" ]; then
    BACKUP_FILE="$CARGO_FILE.backup"
    cp "$CARGO_FILE" "$BACKUP_FILE"
    success "已备份原文件到: $BACKUP_FILE"
fi

# 更新版本
info "更新依赖版本..."

# 创建临时文件
TEMP_FILE=$(mktemp)

if echo "$CONTENT" | grep -qE 'world_rules\s*=\s*"[^"]*"'; then
    # 简单版本格式: world_rules = "1.0"
    echo "$CONTENT" | sed 's/world_rules\s*=\s*"[^"]*"/world_rules = "2.0"/' > "$TEMP_FILE"
    success "已更新简单版本格式"
elif echo "$CONTENT" | grep -qE 'world_rules\s*=\s*\{.*version\s*=\s*"[^"]*".*\}'; then
    # 复杂版本格式: world_rules = { version = "1.0", features = [...] }
    echo "$CONTENT" | sed -E 's/(world_rules\s*=\s*\{[^}]*version\s*=\s*")([^"]*)("\s*[^}]*\})/\12.0\3/' > "$TEMP_FILE"
    success "已更新复杂版本格式"
else
    warning "未找到匹配的版本格式,请手动更新"
    rm "$TEMP_FILE"
    exit 1
fi

# 写入文件
if [ "$DRY_RUN" != "true" ]; then
    mv "$TEMP_FILE" "$CARGO_FILE"
    success "已更新 Cargo.toml"
else
    info "预览更新内容:"
    cat "$TEMP_FILE" | sed 's/^/  /'
    rm "$TEMP_FILE"
fi

# 更新 Cargo.lock
if [ "$DRY_RUN" != "true" ]; then
    info ""
    info "更新 Cargo.lock..."
    cd "$PROJECT_PATH"
    
    if cargo update world_rules 2>&1; then
        success "已更新 Cargo.lock"
    else
        warning "Cargo.lock 更新失败"
    fi
    
    cd - > /dev/null
fi

# 检查编译
info ""
info "检查编译..."
cd "$PROJECT_PATH"

if cargo check 2>&1; then
    success "编译检查通过"
else
    error "编译检查失败"
    if [ "$VERBOSE" = "true" ]; then
        cargo check 2>&1
    fi
    warning "请手动检查编译错误"
fi

cd - > /dev/null

# 运行测试（可选）
info ""
info "检查测试编译..."
cd "$PROJECT_PATH"

if cargo test --no-run 2>&1; then
    success "测试编译通过"
else
    warning "测试编译失败"
    if [ "$VERBOSE" = "true" ]; then
        cargo test --no-run 2>&1
    fi
fi

cd - > /dev/null

# 总结
info ""
info "========================================"
info "迁移完成!"
info "========================================"
info ""

echo "后续步骤:"
echo "  1. 运行完整测试: cargo test"
echo "  2. 检查代码质量: cargo clippy"
echo "  3. 查看变更日志: docs/MIGRATION_GUIDE.md"
echo "  4. 如有问题,恢复备份: $CARGO_FILE.backup"
echo ""

# 检查是否有新功能可用
info "v2.0 新功能提示:"
echo "  - 性能检查系统: PerformanceChecker"
echo "  - 规则难度分级: Difficulty"
echo "  - 更多游戏和法律规则"
echo "  - 性能提升约 20-25%"
echo ""

if [ "$DRY_RUN" = "true" ]; then
    warning "这是 dry run,未实际修改文件"
    info "运行不带 DRY_RUN=true 来执行实际迁移"
fi