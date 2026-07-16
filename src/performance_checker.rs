//! 性能回归检测工具
//! 用于建立性能基线、对比性能变化、生成报告

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 性能基准数据
///
/// 存储单个基准测试的性能数据，包括平均执行时间、标准差和样本数。
/// 用于建立性能基线，以便后续进行性能回归检测。
///
/// # 示例
///
/// ```
/// use world_rules::PerformanceBaseline;
///
/// // 创建性能基准
/// let baseline = PerformanceBaseline {
///     name: "mahjong_validation".to_string(),
///     avg_time_ns: 1500.0,
///     std_dev: 100.0,
///     samples: 100,
///     created_at: "2026-07-16T00:00:00Z".to_string(),
/// };
///
/// assert_eq!(baseline.name, "mahjong_validation");
/// assert_eq!(baseline.avg_time_ns, 1500.0);
/// assert_eq!(baseline.samples, 100);
/// ```
///
/// # 序列化
///
/// 该类型支持 JSON 序列化和反序列化，用于持久化存储：
///
/// ```
/// use world_rules::PerformanceBaseline;
///
/// let baseline = PerformanceBaseline {
///     name: "test".to_string(),
///     avg_time_ns: 1000.0,
///     std_dev: 50.0,
///     samples: 50,
///     created_at: "2026-07-16".to_string(),
/// };
///
/// // 序列化为 JSON
/// let json = serde_json::to_string(&baseline).unwrap();
/// assert!(json.contains("test"));
///
/// // 从 JSON 反序列化
/// let decoded: PerformanceBaseline = serde_json::from_str(&json).unwrap();
/// assert_eq!(decoded.name, baseline.name);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// 基准名称
    ///
    /// 用于标识基准测试的唯一名称，如 "mahjong_validation" 或 "poker_evaluation"。
    pub name: String,
    /// 平均执行时间（纳秒）
    ///
    /// 多次测量后的平均执行时间，单位为纳秒。
    /// 建议至少测量 100 次以获得稳定的平均值。
    pub avg_time_ns: f64,
    /// 标准差
    ///
    /// 执行时间的标准差，反映测量的稳定性。
    /// 标准差越小，说明测量越稳定。
    pub std_dev: f64,
    /// 样本数
    ///
    /// 用于计算平均值的测量次数。
    /// 样本数越多，平均值越可靠。
    pub samples: usize,
    /// 创建时间
    ///
    /// 基准数据的创建时间，使用 ISO 8601 格式。
    pub created_at: String,
}

/// 性能对比结果
///
/// 表示当前性能与基线性能的对比结果，用于判断是否存在性能回归。
///
/// # 示例
///
/// ```
/// use world_rules::PerformanceComparison;
///
/// // 创建性能对比结果
/// let comparison = PerformanceComparison {
///     name: "validation_bench".to_string(),
///     baseline_ns: 1000.0,
///     current_ns: 1100.0,
///     change_percent: 10.0,
///     is_regression: false,
/// };
///
/// assert_eq!(comparison.name, "validation_bench");
/// assert_eq!(comparison.change_percent, 10.0);
/// assert!(!comparison.is_regression); // 10% 变化未超过默认阈值
/// ```
///
/// # 回归判定
///
/// 当 `change_percent` 超过配置的阈值（默认 10%）时，`is_regression` 为 `true`：
///
/// ```
/// use world_rules::PerformanceComparison;
///
/// let regression = PerformanceComparison {
///     name: "slow_bench".to_string(),
///     baseline_ns: 1000.0,
///     current_ns: 1500.0,
///     change_percent: 50.0,
///     is_regression: true,
/// };
///
/// assert!(regression.is_regression);
/// ```
#[derive(Debug)]
pub struct PerformanceComparison {
    /// 基准名称
    ///
    /// 与 [`PerformanceBaseline::name`] 对应的基准测试名称。
    pub name: String,
    /// 基线性能（纳秒）
    ///
    /// 之前测量的基准平均执行时间。
    pub baseline_ns: f64,
    /// 当前性能（纳秒）
    ///
    /// 本次测量的平均执行时间。
    pub current_ns: f64,
    /// 变化百分比
    ///
    /// 计算公式：`(current_ns - baseline_ns) / baseline_ns * 100.0`
    ///
    /// - 正值表示性能下降（变慢）
    /// - 负值表示性能提升（变快）
    pub change_percent: f64,
    /// 是否为回归（性能下降）
    ///
    /// 当 `change_percent` 超过配置的阈值时为 `true`。
    /// 默认阈值为 10%。
    pub is_regression: bool,
}

/// 性能回归检测器
///
/// 用于建立性能基线、对比性能变化和检测性能回归。
///
/// # 示例
///
/// ```
/// use world_rules::{PerformanceChecker, PerformanceBaseline};
///
/// // 创建检测器并加载已有基线
/// let mut checker = PerformanceChecker::new();
/// // 注意：在测试中不加载文件，仅演示 API
///
/// // 添加新的基线数据
/// let baseline = PerformanceBaseline {
///     name: "validation_test".to_string(),
///     avg_time_ns: 1000.0,
///     std_dev: 50.0,
///     samples: 100,
///     created_at: "2026-07-16T00:00:00Z".to_string(),
/// };
/// checker.update_baseline(baseline);
///
/// // 对比性能
/// let comparison = checker.compare_performance("validation_test", 1050.0);
/// assert!(comparison.is_some());
///
/// let comp = comparison.unwrap();
/// assert_eq!(comp.change_percent, 5.0);
/// assert!(!comp.is_regression); // 5% 未超过默认阈值
/// ```
///
/// # 持久化
///
/// 基线数据可以保存到文件并后续加载：
///
/// ```no_run
/// use world_rules::{PerformanceChecker, PerformanceBaseline};
///
/// let mut checker = PerformanceChecker::new();
///
/// // 添加基线
/// checker.update_baseline(PerformanceBaseline {
///     name: "bench1".to_string(),
///     avg_time_ns: 1000.0,
///     std_dev: 100.0,
///     samples: 50,
///     created_at: "2026-07-16".to_string(),
/// });
///
/// // 保存到 .performance/baselines.json
/// checker.save_baselines().unwrap();
///
/// // 后续加载
/// let mut checker2 = PerformanceChecker::new();
/// checker2.load_baselines().unwrap();
/// ```
pub struct PerformanceChecker {
    /// 基线数据
    ///
    /// 存储所有基准测试的基线性能数据，以名称为键。
    baselines: HashMap<String, PerformanceBaseline>,
    /// 配置路径
    ///
    /// 基线数据的存储路径，默认为 `.performance/baselines.json`。
    config_path: PathBuf,
    /// 回归阈值（百分比）
    ///
    /// 当性能变化超过此阈值时判定为回归。
    /// 默认为 10.0（即 10%）。
    regression_threshold: f64,
}

impl PerformanceChecker {
    /// 创建新的性能检测器
    ///
    /// 初始化一个空的检测器，配置路径为默认值。
    ///
    /// # 示例
    ///
    /// ```
    /// use world_rules::PerformanceChecker;
    ///
    /// let checker = PerformanceChecker::new();
    /// assert!(checker.all_baselines().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            baselines: HashMap::new(),
            config_path: PathBuf::from(".performance/baselines.json"),
            regression_threshold: 10.0,
        }
    }

    /// 加载已有基线
    ///
    /// 从配置文件加载之前保存的基线数据。
    /// 如果文件不存在，则保持为空。
    ///
    /// # 错误
    ///
    /// 如果文件存在但格式错误，返回错误。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use world_rules::PerformanceChecker;
    ///
    /// let mut checker = PerformanceChecker::new();
    /// checker.load_baselines().unwrap();
    /// ```
    pub fn load_baselines(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)?;
            self.baselines = serde_json::from_str(&content)?;
        }
        Ok(())
    }

    /// 保存基线数据
    ///
    /// 将当前所有基线数据保存到配置文件。
    /// 如果目录不存在，会自动创建。
    ///
    /// # 错误
    ///
    /// 如果无法创建目录或写入文件，返回错误。
    ///
    /// # Panics
    ///
    /// 不会 panic。所有错误都通过 `Result` 返回。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use world_rules::{PerformanceChecker, PerformanceBaseline};
    ///
    /// let mut checker = PerformanceChecker::new();
    /// checker.update_baseline(PerformanceBaseline {
    ///     name: "test".to_string(),
    ///     avg_time_ns: 1000.0,
    ///     std_dev: 50.0,
    ///     samples: 100,
    ///     created_at: "2026-07-16".to_string(),
    /// });
    /// checker.save_baselines().unwrap();
    /// ```
    pub fn save_baselines(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 使用 ok_or 而不是 unwrap，避免 panic
        let parent = self
            .config_path
            .parent()
            .ok_or_else(|| format!("无效的配置路径: {:?}", self.config_path))?;
        fs::create_dir_all(parent)?;

        let content = serde_json::to_string_pretty(&self.baselines)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    /// 添加或更新基线
    ///
    /// 添加新的基线数据，如果已存在同名基线则更新。
    ///
    /// # 示例
    ///
    /// ```
    /// use world_rules::{PerformanceChecker, PerformanceBaseline};
    ///
    /// let mut checker = PerformanceChecker::new();
    /// let baseline = PerformanceBaseline {
    ///     name: "test".to_string(),
    ///     avg_time_ns: 1000.0,
    ///     std_dev: 50.0,
    ///     samples: 100,
    ///     created_at: "2026-07-16".to_string(),
    /// };
    /// checker.update_baseline(baseline);
    /// assert_eq!(checker.all_baselines().len(), 1);
    /// ```
    pub fn update_baseline(&mut self, baseline: PerformanceBaseline) {
        self.baselines.insert(baseline.name.clone(), baseline);
    }

    /// 对比性能
    ///
    /// 将当前性能与基线性能对比，返回对比结果。
    ///
    /// # 返回值
    ///
    /// - `Some(PerformanceComparison)` - 如果找到对应的基线
    /// - `None` - 如果没有对应的基线
    ///
    /// # Panics
    ///
    /// 当基线的 `avg_time_ns` 为 0 时，除法运算会导致 panic。
    /// 在正常使用情况下不会发生，因为有效基准的执行时间不会为 0。
    ///
    /// # 示例
    ///
    /// ```
    /// use world_rules::{PerformanceChecker, PerformanceBaseline};
    ///
    /// let mut checker = PerformanceChecker::new();
    /// checker.update_baseline(PerformanceBaseline {
    ///     name: "bench".to_string(),
    ///     avg_time_ns: 1000.0,
    ///     std_dev: 50.0,
    ///     samples: 100,
    ///     created_at: "2026-07-16".to_string(),
    /// });
    ///
    /// let comp = checker.compare_performance("bench", 1200.0);
    /// assert!(comp.is_some());
    ///
    /// let comp = comp.unwrap();
    /// assert_eq!(comp.change_percent, 20.0);
    /// assert!(comp.is_regression); // 20% 超过默认阈值
    /// ```
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
    ///
    /// 返回所有已注册的基线数据。
    ///
    /// # 示例
    ///
    /// ```
    /// use world_rules::PerformanceChecker;
    ///
    /// let checker = PerformanceChecker::new();
    /// let baselines = checker.all_baselines();
    /// assert!(baselines.is_empty());
    /// ```
    pub fn all_baselines(&self) -> &HashMap<String, PerformanceBaseline> {
        &self.baselines
    }

    /// 设置回归阈值
    ///
    /// 配置性能回归判定的阈值百分比。
    ///
    /// # 参数
    ///
    /// - `threshold` - 阈值百分比，例如 10.0 表示 10%
    ///
    /// # 示例
    ///
    /// ```
    /// use world_rules::PerformanceChecker;
    ///
    /// let mut checker = PerformanceChecker::new();
    /// checker.set_regression_threshold(15.0); // 设置阈值为 15%
    /// ```
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
///
/// 用于生成性能对比报告，支持 Markdown 和 HTML 格式。
///
/// # 示例
///
/// ```
/// use world_rules::{PerformanceReport, PerformanceComparison};
///
/// let comparisons = vec![
///     PerformanceComparison {
///         name: "validation".to_string(),
///         baseline_ns: 1000.0,
///         current_ns: 1050.0,
///         change_percent: 5.0,
///         is_regression: false,
///     },
/// ];
///
/// // 生成 Markdown 报告
/// let markdown = PerformanceReport::generate_markdown(&comparisons);
/// assert!(markdown.contains("性能回归检测报告"));
/// assert!(markdown.contains("validation"));
///
/// // 生成 HTML 报告
/// let html = PerformanceReport::generate_html(&comparisons);
/// assert!(html.contains("<title>性能回归检测报告</title>"));
/// ```
pub struct PerformanceReport;

impl PerformanceReport {
    /// 生成 Markdown 格式的性能报告
    ///
    /// 创建包含性能对比结果的 Markdown 表格。
    ///
    /// # 输出格式
    ///
    /// ```markdown
    /// # 性能回归检测报告
    ///
    /// | 基准测试 | 基线 (ns) | 当前 (ns) | 变化 | 状态 |
    /// |----------|-----------|-----------|------|------|
    /// | bench1   | 1000.00   | 1050.00   | +5.00%  | ✅ 正常 |
    /// ```
    ///
    /// # 示例
    ///
    /// ```
    /// use world_rules::{PerformanceReport, PerformanceComparison};
    ///
    /// let comparisons = vec![
    ///     PerformanceComparison {
    ///         name: "bench1".to_string(),
    ///         baseline_ns: 1000.0,
    ///         current_ns: 1050.0,
    ///         change_percent: 5.0,
    ///         is_regression: false,
    ///     },
    /// ];
    ///
    /// let markdown = PerformanceReport::generate_markdown(&comparisons);
    /// assert!(markdown.contains("# 性能回归检测报告"));
    /// assert!(markdown.contains("bench1"));
    /// ```
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
    ///
    /// 创建包含性能对比结果的 HTML 表格，带有样式。
    ///
    /// # 输出格式
    ///
    /// 生成的 HTML 包含：
    /// - 响应式表格布局
    /// - 回归项高亮显示（红色背景）
    /// - 改进项高亮显示（绿色背景）
    /// - 完整的 HTML 文档结构
    ///
    /// # 示例
    ///
    /// ```
    /// use world_rules::{PerformanceReport, PerformanceComparison};
    ///
    /// let comparisons = vec![
    ///     PerformanceComparison {
    ///         name: "bench1".to_string(),
    ///         baseline_ns: 1000.0,
    ///         current_ns: 1200.0,
    ///         change_percent: 20.0,
    ///         is_regression: true,
    ///     },
    /// ];
    ///
    /// let html = PerformanceReport::generate_html(&comparisons);
    /// assert!(html.contains("<title>性能回归检测报告</title>"));
    /// assert!(html.contains("regression")); // 回归项有 CSS 类
    /// ```
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
