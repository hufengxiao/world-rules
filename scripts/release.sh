#!/bin/bash
# World Rules 发布脚本
# 用于创建版本 tag 并触发自动发布到 crates.io

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 获取当前版本
VERSION=$(grep '^version =' Cargo.toml | head -1 | sed 's/.*"\([^"]*\)".*/\1/')

echo -e "${GREEN}=== World Rules 发布脚本 ===${NC}"
echo -e "当前版本: ${YELLOW}${VERSION}${NC}"
echo ""

# 检查工作目录状态
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}错误: 工作目录有未提交的更改${NC}"
    echo "请先提交或暂存所有更改"
    git status
    exit 1
fi

# 检查是否有未推送的提交
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git rev-parse origin/master 2>/dev/null || echo "")

if [ "$LOCAL" != "$REMOTE" ]; then
    echo -e "${YELLOW}警告: 本地有未推送的提交${NC}"
    echo "是否要先推送? (y/n)"
    read -r answer
    if [ "$answer" = "y" ]; then
        git push origin master
    else
        echo "取消发布"
        exit 1
    fi
fi

# 检查 tag 是否已存在
TAG="v${VERSION}"
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${RED}错误: Tag ${TAG} 已存在${NC}"
    echo "请更新 Cargo.toml 中的版本号"
    exit 1
fi

# 运行验证
echo -e "${YELLOW}运行验证检查...${NC}"
echo "1. 检查代码格式..."
cargo fmt --all -- --check

echo "2. 运行 clippy..."
cargo clippy -- -D warnings

echo "3. 检查发布包..."
cargo publish --dry-run

echo ""
echo -e "${GREEN}验证通过!${NC}"
echo ""
echo -e "发布说明:"
echo "  - 版本: ${YELLOW}${TAG}${NC}"
echo "  - 将发布到 crates.io"
echo "  - 将创建 GitHub Release"
echo ""
echo -e "${YELLOW}重要提示:${NC}"
echo "  请确保已在 GitHub 设置中配置 CRATES_IO_TOKEN secret"
echo "  https://github.com/hufengxiao/world-rules/settings/secrets/actions"
echo ""
echo "是否继续创建 tag 并推送? (y/n)"
read -r answer

if [ "$answer" = "y" ]; then
    echo -e "${YELLOW}创建 tag ${TAG}...${NC}"
    git tag "$TAG"
    
    echo -e "${YELLOW}推送 tag 到远程仓库...${NC}"
    git push origin "$TAG"
    
    echo ""
    echo -e "${GREEN}✅ 发布已触发!${NC}"
    echo "  - GitHub Actions 将自动发布到 crates.io"
    echo "  - 查看进度: https://github.com/hufengxiao/world-rules/actions"
    echo "  - 发布后查看: https://crates.io/crates/world_rules"
else
    echo "取消发布"
fi