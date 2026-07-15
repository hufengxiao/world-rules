#!/usr/bin/env python3
"""
性能阈值检查脚本
解析 criterion 输出并与配置的阈值比较
"""

import json
import sys
import os
import re
from pathlib import Path

def load_thresholds():
    """加载性能阈值配置"""
    config_path = Path(__file__).parent.parent / "performance-thresholds.json"
    if config_path.exists():
        with open(config_path) as f:
            return json.load(f)
    return {"thresholds": {}, "global": {}}

def parse_criterion_output(output_file):
    """解析 criterion 输出文件"""
    results = {}
    
    if not Path(output_file).exists():
        return results
    
    with open(output_file) as f:
        content = f.read()
    
    # 解析时间数据
    # 格式: test bench_name ... bench: 123.45 ns/iter (+/- 12.34)
    pattern = r"(\w+[\w/]*)\s+time:\s+\[([^\]]+)\]"
    
    for match in re.finditer(pattern, content):
        bench_name = match.group(1)
        time_range = match.group(2)
        # 提取数值
        time_match = re.search(r"([\d.]+)\s*(ns|µs|ms)", time_range)
        if time_match:
            value = float(time_match.group(1))
            unit = time_match.group(2)
            # 转换为微秒
            if unit == "ns":
                value /= 1000
            elif unit == "ms":
                value *= 1000
            results[bench_name] = {"time_us": value}
    
    return results

def check_regression(bench_name, current_time_us, thresholds_config):
    """检查性能是否超过阈值"""
    thresholds = thresholds_config.get("thresholds", {})
    global_config = thresholds_config.get("global", {})
    
    # 查找匹配的阈值配置
    threshold = None
    for key in thresholds:
        if key in bench_name or bench_name.startswith(key):
            threshold = thresholds[key]
            break
    
    if not threshold:
        # 使用默认阈值
        threshold = {
            "regression_percent": global_config.get("default_regression_percent", 10),
            "critical_percent": global_config.get("default_critical_percent", 20)
        }
    
    max_time = threshold.get("max_time_us")
    if max_time and current_time_us > max_time:
        return "critical", f"绝对时间超限: {current_time_us:.2f}us > {max_time}us"
    
    return None, None

def check_json_results(json_file, thresholds_config):
    """检查 JSON 格式的基准测试结果"""
    if not Path(json_file).exists():
        return []
    
    issues = []
    
    try:
        with open(json_file) as f:
            data = json.load(f)
        
        for bench_name, bench_data in data.items():
            if isinstance(bench_data, dict) and "mean" in bench_data:
                time_estimate = bench_data["mean"]
                if isinstance(time_estimate, dict):
                    # criterion JSON 格式: {"estimate": value, "unit": "ns"}
                    value = time_estimate.get("estimate", 0)
                    unit = time_estimate.get("unit", "ns")
                    
                    # 转换为微秒
                    if unit == "ns":
                        value /= 1000
                    elif unit == "ms":
                        value *= 1000
                    
                    status, message = check_regression(bench_name, value, thresholds_config)
                    if status:
                        issues.append({
                            "bench": bench_name,
                            "status": status,
                            "message": message,
                            "value_us": value
                        })
    except json.JSONDecodeError:
        pass
    
    return issues

def generate_report(issues, thresholds_config):
    """生成性能报告"""
    report = ["## 性能阈值检查报告", ""]
    
    if not issues:
        report.append("✅ 所有基准测试均在阈值范围内")
        return "\n".join(report)
    
    critical_count = sum(1 for i in issues if i["status"] == "critical")
    warning_count = sum(1 for i in issues if i["status"] == "warning")
    
    if critical_count > 0:
        report.append(f"🔴 发现 {critical_count} 个严重性能问题")
    if warning_count > 0:
        report.append(f"🟡 发现 {warning_count} 个性能警告")
    
    report.append("")
    report.append("### 详细信息")
    report.append("")
    
    for issue in issues:
        icon = "🔴" if issue["status"] == "critical" else "🟡"
        report.append(f"{icon} **{issue['bench']}**: {issue['message']}")
    
    return "\n".join(report)

def main():
    """主函数"""
    thresholds_config = load_thresholds()
    
    # 检查基准测试结果
    results_dir = Path(__file__).parent.parent.parent / ".bench-results"
    
    issues = []
    
    # 检查 JSON 结果
    json_file = results_dir / "results.json"
    if json_file.exists():
        issues.extend(check_json_results(str(json_file), thresholds_config))
    
    # 生成报告
    report = generate_report(issues, thresholds_config)
    print(report)
    
    # 输出结果到文件
    report_file = results_dir / "threshold-report.md"
    with open(report_file, "w") as f:
        f.write(report)
    
    # 如果有严重问题，返回错误码
    critical_count = sum(1 for i in issues if i["status"] == "critical")
    if critical_count > 0 and thresholds_config.get("global", {}).get("fail_on_critical", True):
        sys.exit(1)
    
    sys.exit(0)

if __name__ == "__main__":
    main()