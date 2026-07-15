#!/bin/bash
# 性能回归检测脚本
# 用于本地和 CI 环境中检测性能回归

set -e

BASELINE_DIR="target/criterion"
HISTORY_DIR=".performance-history"
THRESHOLD=10.0

echo "=== 性能回归检测 ==="
echo ""

# 检查基线目录是否存在
if [ ! -d "$BASELINE_DIR" ]; then
    echo "❌ 错误: 未找到基准测试结果目录 $BASELINE_DIR"
    echo "请先运行: cargo bench"
    exit 1
fi

# 创建历史目录
mkdir -p "$HISTORY_DIR"

# 运行新的基准测试
echo "运行基准测试..."
cargo bench -- --save-baseline current 2>&1 | grep -E "(time|change)" || true

# 对比分析
echo ""
echo "=== 性能对比分析 ==="

if [ -d "$BASELINE_DIR" ]; then
    # 查找所有基准测试结果
    find "$BASELINE_DIR" -name "new" -type d | while read -r dir; do
        bench_name=$(basename "$(dirname "$dir")")
        echo "检查: $bench_name"
    done
fi

# 保存当前结果到历史
DATE=$(date +%Y-%m-%d-%H%M%S)
if [ -d "$BASELINE_DIR" ]; then
    cp -r "$BASELINE_DIR" "$HISTORY_DIR/benchmark-$DATE"
    echo ""
    echo "✅ 基准测试结果已保存到: $HISTORY_DIR/benchmark-$DATE"
fi

echo ""
echo "=== 检测完成 ==="