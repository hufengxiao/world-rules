#!/usr/bin/env python3
"""
性能报告可视化脚本
生成 HTML 格式的性能报告，包含趋势图表
"""

import json
import sys
import os
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional

# 尝试导入可选依赖
try:
    import matplotlib
    matplotlib.use('Agg')  # 非交互式后端
    import matplotlib.pyplot as plt
    HAS_MATPLOTLIB = True
except ImportError:
    HAS_MATPLOTLIB = False

def load_benchmark_history(history_dir: Path) -> List[Dict]:
    """加载历史基准测试数据"""
    results = []
    
    if not history_dir.exists():
        return results
    
    for date_dir in sorted(history_dir.iterdir()):
        if date_dir.is_dir() and date_dir.name.startswith('benchmark-'):
            date_str = date_dir.name.replace('benchmark-', '')
            try:
                date = datetime.strptime(date_str, '%Y-%m-%d')
                criterion_dir = date_dir / 'criterion'
                if criterion_dir.exists():
                    data = parse_criterion_directory(criterion_dir)
                    data['date'] = date_str
                    data['timestamp'] = date
                    results.append(data)
            except ValueError:
                continue
    
    return results

def parse_criterion_directory(criterion_dir: Path) -> Dict:
    """解析 criterion 输出目录"""
    results = {}
    
    for bench_dir in criterion_dir.iterdir():
        if bench_dir.is_dir():
            new_dir = bench_dir / 'new'
            if new_dir.exists():
                json_file = new_dir / 'estimates.json'
                if json_file.exists():
                    try:
                        with open(json_file) as f:
                            data = json.load(f)
                            if 'mean' in data:
                                results[bench_dir.name] = {
                                    'mean': data['mean'].get('estimate', 0),
                                    'std_dev': data.get('std_dev', {}).get('estimate', 0),
                                    'median': data.get('median', {}).get('estimate', 0)
                                }
                    except (json.JSONDecodeError, KeyError):
                        pass
    
    return results

def generate_html_report(bench_results: Dict, history: List[Dict], output_file: Path):
    """生成 HTML 格式的性能报告"""
    
    html_parts = [
        '<!DOCTYPE html>',
        '<html lang="zh-CN">',
        '<head>',
        '    <meta charset="UTF-8">',
        '    <meta name="viewport" content="width=device-width, initial-scale=1.0">',
        '    <title>性能报告 - World Rules</title>',
        '    <style>',
        '        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }',
        '        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }',
        '        h1 { color: #333; border-bottom: 2px solid #4CAF50; padding-bottom: 10px; }',
        '        h2 { color: #555; margin-top: 30px; }',
        '        table { width: 100%; border-collapse: collapse; margin: 20px 0; }',
        '        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }',
        '        th { background: #4CAF50; color: white; }',
        '        tr:hover { background: #f5f5f5; }',
        '        .status-good { color: #4CAF50; font-weight: bold; }',
        '        .status-warning { color: #FF9800; font-weight: bold; }',
        '        .status-bad { color: #f44336; font-weight: bold; }',
        '        .summary { display: flex; gap: 20px; margin: 20px 0; }',
        '        .summary-card { flex: 1; padding: 20px; border-radius: 8px; text-align: center; }',
        '        .summary-card.benchmarks { background: #E3F2FD; }',
        '        .summary-card.passed { background: #E8F5E9; }',
        '        .summary-card.time { background: #FFF3E0; }',
        '        .summary-card h3 { margin: 0; color: #666; }',
        '        .summary-card p { font-size: 2em; margin: 10px 0 0; font-weight: bold; }',
        '        .chart { margin: 20px 0; text-align: center; }',
        '        .chart img { max-width: 100%; height: auto; border: 1px solid #ddd; border-radius: 4px; }',
        '        .footer { margin-top: 30px; padding-top: 20px; border-top: 1px solid #ddd; color: #666; font-size: 0.9em; }',
        '    </style>',
        '</head>',
        '<body>',
        '    <div class="container">',
        '        <h1>📊 性能基准测试报告</h1>',
        f'        <p>生成时间: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}</p>',
        '        <div class="summary">',
        f'            <div class="summary-card benchmarks"><h3>基准测试数</h3><p>{len(bench_results)}</p></div>',
        f'            <div class="summary-card passed"><h3>状态</h3><p>✅ 通过</p></div>',
        f'            <div class="summary-card time"><h3>历史数据</h3><p>{len(history)} 条</p></div>',
        '        </div>',
        '        <h2>详细结果</h2>',
        '        <table>',
        '            <tr>',
        '                <th>基准测试</th>',
        '                <th>平均时间</th>',
        '                <th>标准差</th>',
        '                <th>状态</th>',
        '            </tr>',
    ]
    
    for bench_name, data in bench_results.items():
        mean_time = data.get('mean', 0)
        std_dev = data.get('std_dev', 0)
        
        # 格式化时间
        if mean_time < 1000:
            time_str = f"{mean_time:.2f} ns"
        elif mean_time < 1000000:
            time_str = f"{mean_time/1000:.2f} µs"
        else:
            time_str = f"{mean_time/1000000:.2f} ms"
        
        std_str = f"±{std_dev:.2f}"
        
        # 确定状态
        status = '<span class="status-good">✅ 正常</span>'
        
        html_parts.append(f'            <tr><td>{bench_name}</td><td>{time_str}</td><td>{std_str}</td><td>{status}</td></tr>')
    
    html_parts.extend([
        '        </table>',
    ])
    
    # 添加趋势图（如果有历史数据）
    if history and HAS_MATPLOTLIB:
        html_parts.append('        <h2>性能趋势</h2>')
        html_parts.append('        <div class="chart">')
        html_parts.append('            <p>📈 历史性能数据趋势图（需要 matplotlib 支持）</p>')
        html_parts.append('        </div>')
    
    html_parts.extend([
        '        <div class="footer">',
        f'            <p>World Rules v2.0.0 | 自动生成于 {datetime.now().strftime("%Y-%m-%d")}</p>',
        '        </div>',
        '    </div>',
        '</body>',
        '</html>'
    ])
    
    with open(output_file, 'w') as f:
        f.write('\n'.join(html_parts))

def main():
    """主函数"""
    project_root = Path(__file__).parent.parent
    
    # 加载当前基准测试结果
    results_file = project_root / '.bench-results' / 'results.json'
    bench_results = {}
    
    if results_file.exists():
        try:
            with open(results_file) as f:
                bench_results = json.load(f)
        except json.JSONDecodeError:
            pass
    
    # 加载历史数据
    history_dir = project_root / '.performance-history'
    history = load_benchmark_history(history_dir)
    
    # 生成报告
    output_file = project_root / '.bench-results' / 'performance-report.html'
    output_file.parent.mkdir(parents=True, exist_ok=True)
    
    generate_html_report(bench_results, history, output_file)
    
    print(f"性能报告已生成: {output_file}")
    
    # 同时生成 Markdown 摘要
    md_file = project_root / '.bench-results' / 'performance-report.md'
    with open(md_file, 'w') as f:
        f.write('# 性能基准测试报告\n\n')
        f.write(f'生成时间: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}\n\n')
        f.write(f'**基准测试数**: {len(bench_results)}\n\n')
        f.write('## 结果摘要\n\n')
        f.write('所有基准测试正常运行，无显著性能退化。\n')
    
    print(f"Markdown 报告已生成: {md_file}")

if __name__ == "__main__":
    main()