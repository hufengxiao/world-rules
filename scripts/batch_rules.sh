#!/usr/bin/env bash
# 批量生成+验证规则
# 用法: scripts/batch_rules.sh <specs目录> <分类>
set -euo pipefail
SPECS_DIR="$1"
CAT="${2:-social}"
cd "$(dirname "$0")/.."

# 空目录保护
shopt -s nullglob
specs=("$SPECS_DIR"/_spec_*.json)
echo "待处理规格数: ${#specs[@]}"

count=0
for spec in "${specs[@]}"; do
    echo ">>> $(basename "$spec")"
    python3 scripts/wrgen.py < "$spec"
    count=$((count+1))
done
echo "生成的规则模块数: $count"

export PATH="/root/.cargo_rust/bin:$PATH"
export CARGO_HOME=/root/.cargo_home
echo "=== cargo build ==="
cargo build 2>&1 | tail -8
echo "=== cargo test (rules::$CAT) ==="
cargo test --lib "rules::${CAT}" 2>&1 | tail -4
echo "=== cargo fmt ==="
cargo fmt
echo "=== DONE ==="