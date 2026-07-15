#!/usr/bin/env python3
"""
自动性能对比脚本
对比当前基准测试结果与历史基线
"""

import json
import sys
import os
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple

# 性能变化阈值
REGRESSION_THRESHOLD = 10  # 超过 10% 视为性能退化
IMPROVEMENT_THRESHOLD = 10  # 超过 10% 视为性能改进

def load_baseline(baseline_dir: Path) -> Dict:
    """加载基线性能数据"""
    baseline_file = baseline_dir / 'results.json'
    if baseline_file.exists():
        try:
            with open(baseline_file) as f:
                return json.load(f)
        except json.JSONDecodeError:
            pass
    return {}

def load_current_results(results_dir: Path) -> Dict:
    """加载当前性能结果"""
    results_file = results_dir / 'results.json'
    if results_file.exists():
        try:
            with open(results_file) as f:
                return json.load(f)
        except json.JSONDecodeError:
            pass
    return {}

def parse_time_value(value_str: str) -> float:
    """解析时间字符串为纳秒"""
    if isinstance(value_str, (int, float)):
        return float(value_str)
    
    if not isinstance(value_str, str):
        return 0.0
    
    value_str = value_str.strip()
    
    # 尝试解析数值和单位
    import re
    match = re.match(r'([\d.]+)\s*(ns|µs|us|ms|s)?', value_str, re.IGNORECASE)
    if match:
        value = float(match.group(1))
        unit = match.group(2) or 'ns'
        
        # 转换为纳秒
        if unit.lower() == 'ns':
            return value
        elif unit.lower() in ('µs', 'us'):
            return value * 1000
        elif unit.lower() == 'ms':
            return value * 1000000
        elif unit.lower() == 's':
            return value * 1000000000
    
    return 0.0

def compare_benchmarks(current: Dict, baseline: Dict) -> List[Dict]:
    """对比基准测试结果"""
    comparisons = []
    
    for bench_name, current_data in current.items():
        if bench_name not in baseline:
            comparisons.append({
                'name': bench_name,
                'status': 'new',
                'current': current_data,
                'baseline': None,
                'change_percent': 0
            })
            continue
        
        baseline_data = baseline[bench_name]
        
        # 提取时间值
        current_time = 0.0
        baseline_time = 0.0
        
        if isinstance(current_data, dict):
            if 'mean' in current_data:
                current_time = parse_time_value(current_data['mean'])
            elif 'estimate' in current_data:
                current_time = parse_time_value(current_data['estimate'])
        
        if isinstance(baseline_data, dict):
            if 'mean' in baseline_data:
                baseline_time = parse_time_value(baseline_data['mean'])
            elif 'estimate' in baseline_data:
                baseline_time = parse_time_value(baseline_data['estimate'])
        
        # 计算变化百分比
        change_percent = 0.0
        if baseline_time > 0:
            change_percent = ((current_time - baseline_time) / baseline_time) * 100
        
        # 确定状态
        status = 'unchanged'
        if change_percent > REGRESSION_THRESHOLD:
            status = 'regression'
        elif change_percent < -IMPROVEMENT_THRESHOLD:
            status = 'improvement'
        
        comparisons.append({
            'name': bench_name,
            'status': status,
            'current_time_ns': current_time,
            'baseline_time_ns': baseline_time,
            'change_percent': round(change_percent, 2)
        })
    
    return comparisons

def generate_comparison_report(comparisons: List[Dict]) -> str:
    """生成对比报告"""
    lines = [
        '# 性能对比报告',
        '',
        f'生成时间: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}',
        '',
        '## 对比结果摘要',
        ''
    ]
    
    # 统计
    regressions = [c for c in comparisons if c['status'] == 'regression']
    improvements = [c for c in comparisons if c['status'] == 'improvement']
    unchanged = [c for c in comparisons if c['status'] == 'unchanged']
    new = [c for c in comparisons if c['status'] == 'new']
    
    lines.append(f'- 🔴 性能退化: {len(regressions)} 项')
    lines.append(f'- 🟢 性能改进: {len(improvements)} 项')
    lines.append(f'- ✅ 无变化: {len(unchanged)} 项')
    lines.append(f'- 🆕 新测试: {len(new)} 项')
    lines.append('')
    
    # 详细结果
    if regressions:
        lines.append('## 🔴 性能退化详情')
        lines.append('')
        lines.append('| 基准测试 | 当前时间 | 基线时间 | 变化 |')
        lines.append('|----------|----------|----------|------|')
        for c in regressions:
            lines.append(f'| {c["name"]} | {c["current_time_ns"]:.2f}ns | {c["baseline_time_ns"]:.2f}ns | +{c["change_percent"]}% |')
        lines.append('')
    
    if improvements:
        lines.append('## 🟢 性能改进详情')
        lines.append('')
        lines.append('| 基准测试 | 当前时间 | 基线时间 | 变化 |')
        lines.append('|----------|----------|----------|------|')
        for c in improvements:
            lines.append(f'| {c["name"]} | {c["current_time_ns"]:.2f}ns | {c["baseline_time_ns"]:.2f}ns | {c["change_percent"]}% |')
        lines.append('')
    
    # 总结
    lines.append('## 总结')
    lines.append('')
    if regressions:
        lines.append(f'⚠️ **检测到 {len(regressions)} 项性能退化，需要关注！**')
    else:
        lines.append('✅ **未检测到性能退化，系统性能稳定。**')
    lines.append('')
    
    return '\n'.join(lines)

def main():
    """主函数"""
    project_root = Path(__file__).parent.parent
    results_dir = project_root / '.bench-results'
    baseline_dir = results_dir / 'base'
    
    # 加载数据
    current = load_current_results(results_dir)
    baseline = load_baseline(baseline_dir)
    
    if not current:
        print("警告: 未找到当前基准测试结果")
        sys.exit(0)
    
    if not baseline:
        print("警告: 未找到基线性能数据")
        # 仍然生成报告，只是没有对比数据
        comparisons = []
        for name, data in current.items():
            comparisons.append({
                'name': name,
                'status': 'new',
                'current_time_ns': 0,
                'baseline_time_ns': 0,
                'change_percent': 0
            })
    else:
        # 对比
        comparisons = compare_benchmarks(current, baseline)
    
    # 生成报告
    report = generate_comparison_report(comparisons)
    
    # 输出报告
    output_file = results_dir / 'comparison-report.md'
    with open(output_file, 'w') as f:
        f.write(report)
    
    print(f"对比报告已生成: {output_file}")
    print()
    print(report)
    
    # 如果有性能退化，返回错误码
    regressions = [c for c in comparisons if c['status'] == 'regression']
    if regressions:
        print()
        print("⚠️ 检测到性能退化！")
        sys.exit(1)
    
    sys.exit(0)

if __name__ == "__main__":
    main()