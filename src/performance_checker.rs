//! 性能回归检测工具
//! 用于建立性能基线、对比性能变化、生成报告

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 性能基准数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// 基准名称
    pub name: String,
    /// 平均执行时间（纳秒）
    pub avg_time_ns: f64,
    /// 标准差
    pub std_dev: f64,
    /// 样本数
    pub samples: usize,
    /// 创建时间
    pub created_at: String,
}

/// 性能对比结果
#[derive(Debug)]
pub struct PerformanceComparison {
    /// 基准名称
    pub name: String,
    /// 基线性能
    pub baseline_ns: f64,
    /// 当前性能
    pub current_ns: f64,
    /// 变化百分比
    pub change_percent: f64,
    /// 是否为回归（性能下降）
    pub is_regression: bool,
}

/// 性能回归检测器
pub struct PerformanceChecker {
    /// 基线数据
    baselines: HashMap<String, PerformanceBaseline>,
    /// 配置路径
    config_path: PathBuf,
    /// 回归阈值（百分比）
    regression_threshold: f64,
}

impl PerformanceChecker {
    /// 创建新的性能检测器
    pub fn new() -> Self {
        Self {
            baselines: HashMap::new(),
            config_path: PathBuf::from(".performance/baselines.json"),
            regression_threshold: 10.0,
        }
    }

    /// 加载已有基线
    pub fn load_baselines(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)?;
            self.baselines = serde_json::from_str(&content)?;
        }
        Ok(())
    }

    /// 保存基线数据
    pub fn save_baselines(&self) -> Result<(), Box<dyn std::error::Error>> {
        let parent = self.config_path.parent().unwrap();
        fs::create_dir_all(parent)?;

        let content = serde_json::to_string_pretty(&self.baselines)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    /// 添加或更新基线
    pub fn update_baseline(&mut self, baseline: PerformanceBaseline) {
        self.baselines.insert(baseline.name.clone(), baseline);
    }

    /// 对比性能
    pub fn compare_performance(
        &self,
        name: &str,
        current_ns: f64,
    ) -> Option<PerformanceComparison> {
        self.baselines.get(name).map(|baseline| {
            let change_percent =
                ((current_ns - baseline.avg_time_ns) / baseline.avg_time_ns) * 100.0;

            PerformanceComparison {
                name: name.to_string(),
                baseline_ns: baseline.avg_time_ns,
                current_ns,
                change_percent,
                is_regression: change_percent > self.regression_threshold,
            }
        })
    }

    /// 获取所有基线
    pub fn all_baselines(&self) -> &HashMap<String, PerformanceBaseline> {
        &self.baselines
    }

    /// 设置回归阈值
    pub fn set_regression_threshold(&mut self, threshold: f64) {
        self.regression_threshold = threshold;
    }
}

impl Default for PerformanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// 性能报告生成器
pub struct PerformanceReport;

impl PerformanceReport {
    /// 生成 Markdown 格式的性能报告
    pub fn generate_markdown(comparisons: &[PerformanceComparison]) -> String {
        let mut report = String::new();
        report.push_str("# 性能回归检测报告\n\n");
        report.push_str("| 基准测试 | 基线 (ns) | 当前 (ns) | 变化 | 状态 |\n");
        report.push_str("|----------|-----------|-----------|------|------|\n");

        for comp in comparisons {
            let status = if comp.is_regression {
                "❌ 回归"
            } else if comp.change_percent < -5.0 {
                "✅ 改进"
            } else {
                "✅ 正常"
            };

            report.push_str(&format!(
                "| {} | {:.2} | {:.2} | {:+.2}% | {} |\n",
                comp.name, comp.baseline_ns, comp.current_ns, comp.change_percent, status
            ));
        }

        report
    }

    /// 生成 HTML 格式的性能报告
    pub fn generate_html(comparisons: &[PerformanceComparison]) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <title>性能回归检测报告</title>\n");
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("    table { border-collapse: collapse; width: 100%; }\n");
        html.push_str("    th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("    th { background-color: #4CAF50; color: white; }\n");
        html.push_str("    .regression { background-color: #ffcccc; }\n");
        html.push_str("    .improvement { background-color: #ccffcc; }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <h1>性能回归检测报告</h1>\n");
        html.push_str("  <table>\n");
        html.push_str("    <tr>\n");
        html.push_str("      <th>基准测试</th>\n");
        html.push_str("      <th>基线 (ns)</th>\n");
        html.push_str("      <th>当前 (ns)</th>\n");
        html.push_str("      <th>变化</th>\n");
        html.push_str("      <th>状态</th>\n");
        html.push_str("    </tr>\n");

        for comp in comparisons {
            let class = if comp.is_regression {
                "regression"
            } else if comp.change_percent < -5.0 {
                "improvement"
            } else {
                ""
            };

            let status = if comp.is_regression {
                "❌ 回归"
            } else if comp.change_percent < -5.0 {
                "✅ 改进"
            } else {
                "✅ 正常"
            };

            html.push_str(&format!("    <tr class=\"{}\">\n", class));
            html.push_str(&format!("      <td>{}</td>\n", comp.name));
            html.push_str(&format!("      <td>{:.2}</td>\n", comp.baseline_ns));
            html.push_str(&format!("      <td>{:.2}</td>\n", comp.current_ns));
            html.push_str(&format!("      <td>{:+.2}%</td>\n", comp.change_percent));
            html.push_str(&format!("      <td>{}</td>\n", status));
            html.push_str("    </tr>\n");
        }

        html.push_str("  </table>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        html
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_baseline_creation() {
        let baseline = PerformanceBaseline {
            name: "test_bench".to_string(),
            avg_time_ns: 1000.0,
            std_dev: 100.0,
            samples: 100,
            created_at: "2026-01-01".to_string(),
        };

        assert_eq!(baseline.name, "test_bench");
        assert_eq!(baseline.avg_time_ns, 1000.0);
    }

    #[test]
    fn test_performance_checker() {
        let mut checker = PerformanceChecker::new();

        let baseline = PerformanceBaseline {
            name: "test".to_string(),
            avg_time_ns: 1000.0,
            std_dev: 100.0,
            samples: 100,
            created_at: "2026-01-01".to_string(),
        };

        checker.update_baseline(baseline);

        let comp = checker.compare_performance("test", 1200.0);
        assert!(comp.is_some());

        let comp = comp.unwrap();
        assert_eq!(comp.change_percent, 20.0);
        assert!(comp.is_regression);
    }

    #[test]
    fn test_report_generation() {
        let comparisons = vec![
            PerformanceComparison {
                name: "bench1".to_string(),
                baseline_ns: 1000.0,
                current_ns: 1050.0,
                change_percent: 5.0,
                is_regression: false,
            },
            PerformanceComparison {
                name: "bench2".to_string(),
                baseline_ns: 2000.0,
                current_ns: 2500.0,
                change_percent: 25.0,
                is_regression: true,
            },
        ];

        let markdown = PerformanceReport::generate_markdown(&comparisons);
        assert!(markdown.contains("性能回归检测报告"));
        assert!(markdown.contains("bench1"));
        assert!(markdown.contains("bench2"));
    }
}
