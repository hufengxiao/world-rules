#!/bin/bash
# 性能基准对比脚本
# 用于比较不同版本的基准测试结果

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RESULTS_DIR="$PROJECT_ROOT/.bench-results"
BASELINE_DIR="$RESULTS_DIR/baseline"
CURRENT_DIR="$RESULTS_DIR/current"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 帮助信息
print_help() {
    echo "用法: $0 [命令] [选项]"
    echo ""
    echo "命令:"
    echo "  run         运行基准测试并保存结果"
    echo "  compare     对比当前结果与基准"
    echo "  save        将当前结果保存为新基准"
    echo "  report      生成性能报告"
    echo "  check       运行基准测试并检查性能退化"
    echo ""
    echo "选项:"
    echo "  --baseline  指定基准目录（默认: .bench-results/baseline）"
    echo "  --output    指定输出文件"
    echo "  --threshold 设置退化阈值百分比（默认: 10）"
    echo ""
    echo "示例:"
    echo "  $0 run                     # 运行基准测试"
    echo "  $0 compare                 # 对比性能"
    echo "  $0 check --threshold 15   # 检查退化（阈值 15%）"
}

# 确保目录存在
ensure_dirs() {
    mkdir -p "$RESULTS_DIR"
    mkdir -p "$BASELINE_DIR"
    mkdir -p "$CURRENT_DIR"
}

# 运行基准测试
run_benchmarks() {
    echo -e "${YELLOW}运行性能基准测试...${NC}"
    ensure_dirs
    
    cd "$PROJECT_ROOT"
    
    # 运行每个基准测试
    for bench in mahjong_bench poker_bench sudoku_bench; do
        echo "  - 运行 $bench..."
        cargo bench --bench "$bench" -- --save-baseline current 2>&1 \
            | tee "$CURRENT_DIR/${bench}.txt" || true
        
        # 提取 JSON 格式的结果
        cargo bench --bench "$bench" -- --message-format=json 2>/dev/null \
            > "$CURRENT_DIR/${bench}.json" || true
    done
    
    # 生成摘要
    echo ""
    echo -e "${GREEN}基准测试完成！${NC}"
    echo "结果保存在: $CURRENT_DIR"
}

# 对比基准结果
compare_results() {
    local threshold=${1:-10}
    local regression_found=false
    
    echo -e "${YELLOW}对比性能基准...${NC}"
    echo ""
    
    if [ ! -d "$BASELINE_DIR" ] || [ -z "$(ls -A $BASELINE_DIR 2>/dev/null)" ]; then
        echo -e "${YELLOW}警告: 未找到基准数据，无法对比${NC}"
        echo "请先运行: $0 save"
        return 0
    fi
    
    echo "| 基准测试 | 基准时间 | 当前时间 | 变化 | 状态 |"
    echo "|----------|----------|----------|------|------|"
    
    for bench in mahjong_bench poker_bench sudoku_bench; do
        baseline_file="$BASELINE_DIR/${bench}.txt"
        current_file="$CURRENT_DIR/${bench}.txt"
        
        if [ -f "$baseline_file" ] && [ -f "$current_file" ]; then
            # 使用 critcmp 或简单的文本解析对比
            if command -v critcmp &> /dev/null; then
                # 使用 critcmp 工具
                critcmp baseline current 2>/dev/null || true
            else
                # 简单的文本对比
                compare_text_results "$bench" "$baseline_file" "$current_file" "$threshold"
            fi
        else
            echo "| $bench | - | - | - | 跳过 |"
        fi
    done
    
    echo ""
    
    if [ "$regression_found" = true ]; then
        echo -e "${RED}⚠️  检测到性能退化！${NC}"
        return 1
    else
        echo -e "${GREEN}✅ 未检测到显著性能退化${NC}"
        return 0
    fi
}

# 文本结果对比
compare_text_results() {
    local bench=$1
    local baseline=$2
    local current=$3
    local threshold=$4
    
    # 提取平均时间（简化版本）
    local baseline_time=$(grep -oP 'time:\s+\K[\d.]+' "$baseline" | head -1 || echo "N/A")
    local current_time=$(grep -oP 'time:\s+\K[\d.]+' "$current" | head -1 || echo "N/A")
    
    if [ "$baseline_time" != "N/A" ] && [ "$current_time" != "N/A" ]; then
        # 计算变化百分比
        local change=$(echo "scale=2; (($current_time - $baseline_time) / $baseline_time) * 100" | bc 2>/dev/null || echo "0")
        
        local status="✅ OK"
        if (( $(echo "$change > $threshold" | bc -l) )); then
            status="⚠️ 退化"
            regression_found=true
        elif (( $(echo "$change < -$threshold" | bc -l) )); then
            status="🚀 改进"
        fi
        
        printf "| %-15s | %10s | %10s | %+6.2f%% | %s |\n" \
            "$bench" "${baseline_time}µs" "${current_time}µs" "$change" "$status"
    else
        printf "| %-15s | %10s | %10s | %6s | %s |\n" "$bench" "N/A" "N/A" "-" "跳过"
    fi
}

# 保存当前结果为基准
save_baseline() {
    echo -e "${YELLOW}保存当前结果为基准...${NC}"
    ensure_dirs
    
    if [ ! -d "$CURRENT_DIR" ] || [ -z "$(ls -A $CURRENT_DIR 2>/dev/null)" ]; then
        echo -e "${RED}错误: 未找到当前基准结果${NC}"
        echo "请先运行: $0 run"
        exit 1
    fi
    
    # 备份旧基准
    if [ -d "$BASELINE_DIR" ] && [ "$(ls -A $BASELINE_DIR 2>/dev/null)" ]; then
        backup_dir="$RESULTS_DIR/baseline_$(date +%Y%m%d_%H%M%S)"
        echo "备份旧基准到: $backup_dir"
        mv "$BASELINE_DIR" "$backup_dir"
        mkdir -p "$BASELINE_DIR"
    fi
    
    # 复制当前结果为基准
    cp -r "$CURRENT_DIR"/* "$BASELINE_DIR/"
    
    # 保存元数据
    echo "$(date -Iseconds)" > "$BASELINE_DIR/timestamp.txt"
    git rev-parse HEAD 2>/dev/null >> "$BASELINE_DIR/timestamp.txt" || echo "unknown" >> "$BASELINE_DIR/timestamp.txt"
    
    echo -e "${GREEN}✅ 基准已保存${NC}"
}

# 生成性能报告
generate_report() {
    local output_file=${1:-"$RESULTS_DIR/benchmark-report.md"}
    
    echo -e "${YELLOW}生成性能报告...${NC}"
    
    cat > "$output_file" << EOF
# 性能基准测试报告

生成时间: $(date -Iseconds)
Git Commit: $(git rev-parse HEAD 2>/dev/null || echo "unknown")

## 测试环境

- 操作系统: $(uname -s) $(uname -r)
- Rust 版本: $(rustc --version 2>/dev/null || echo "unknown")
- CPU: $(lscpu | grep "Model name" | cut -d: -f2 | xargs || echo "unknown")
- 内存: $(free -h | grep Mem | awk '{print $2}' || echo "unknown")

## 基准测试结果

EOF

    for bench in mahjong_bench poker_bench sudoku_bench; do
        current_file="$CURRENT_DIR/${bench}.txt"
        
        if [ -f "$current_file" ]; then
            echo "### $bench" >> "$output_file"
            echo "" >> "$output_file"
            echo '```' >> "$output_file"
            grep -E "(test|time:|change:)" "$current_file" | head -30 >> "$output_file"
            echo '```' >> "$output_file"
            echo "" >> "$output_file"
        fi
    done
    
    # 添加对比信息
    if [ -d "$BASELINE_DIR" ] && [ "$(ls -A $BASELINE_DIR 2>/dev/null)" ]; then
        echo "## 与基准对比" >> "$output_file"
        echo "" >> "$output_file"
        compare_results >> "$output_file" 2>&1 || true
    fi
    
    echo -e "${GREEN}报告已生成: $output_file${NC}"
}

# 检查性能退化
check_regression() {
    local threshold=${1:-10}
    
    echo -e "${YELLOW}检查性能退化（阈值: ${threshold}%）...${NC}"
    
    # 运行基准测试
    run_benchmarks
    
    # 对比结果
    if compare_results "$threshold"; then
        echo -e "${GREEN}✅ 性能检查通过${NC}"
        return 0
    else
        echo -e "${RED}❌ 性能检查失败${NC}"
        return 1
    fi
}

# 主入口
case "${1:-help}" in
    run)
        run_benchmarks
        ;;
    compare)
        compare_results "${3:-10}"
        ;;
    save)
        save_baseline
        ;;
    report)
        generate_report "${3:-$RESULTS_DIR/benchmark-report.md}"
        ;;
    check)
        shift
        threshold=10
        while [[ $# -gt 0 ]]; do
            case $1 in
                --threshold)
                    threshold="$2"
                    shift 2
                    ;;
                *)
                    shift
                    ;;
            esac
        done
        check_regression "$threshold"
        ;;
    help|--help|-h)
        print_help
        ;;
    *)
        echo -e "${RED}未知命令: $1${NC}"
        print_help
        exit 1
        ;;
esac