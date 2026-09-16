//! 统计与数据伦理
//!
//! 数据搜集、呈现与传播中的统计与科学伦理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: StatisticsEthicsRules,
    name: "统计与数据伦理",
    desc: "数据搜集、呈现与传播中的统计与科学伦理规则",
    origin: "国际",
    tags: ["科学", "统计", "伦理", "数据", "严谨"]
}

impl StatisticsEthicsRules {
    /// 样本与收集
    pub fn sampling(&self) -> Vec<&'static str> {
        vec![
            "样本应具代表性避免偏倚",
            "调查过程遵守知情与隐私",
            "不擅自截取有利数据",
            "如实记录收集环境",
        ]
    }

    /// 分析与呈现
    pub fn analysis(&self) -> Vec<&'static str> {
        vec![
            "审慎解释相关性",
            "不混淆因果与相关",
            "图表真实不误导",
            "标注数据来源与误差",
        ]
    }

    /// 结论与发布
    pub fn reporting(&self) -> Vec<&'static str> {
        vec![
            "结论要有统计支撑",
            "不以个别异常泛化",
            "如实说明局限性",
            "不操纵语义误导读者",
        ]
    }

    /// 严谨自律
    pub fn rigor(&self) -> Vec<&'static str> {
        vec![
            "尊重数据原始性与完整性",
            "不篡改清理掉反证数据",
            "复现他人方法时尊重真实性",
            "不伪造或编造数据",
        ]
    }
}

impl Rule for StatisticsEthicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("statistics_ethics")
    }

    fn explain(&self) -> String {
        format!(
            "【统计与数据伦理】\n{}",
            [
                format!(
                    "样本与收集：\\n{}",
                    self.sampling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分析与呈现：\\n{}",
                    self.analysis()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结论与发布：\\n{}",
                    self.reporting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "严谨自律：\\n{}",
                    self.rigor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_statisticsethicsrules_basic() {
        let rules = StatisticsEthicsRules::new();
        assert_eq!(rules.metadata().name, "统计与数据伦理");
        assert!(!rules.sampling().is_empty());
        assert!(!rules.analysis().is_empty());
        assert!(!rules.reporting().is_empty());
        assert!(!rules.rigor().is_empty());
    }

    #[test]
    fn test_statisticsethicsrules_validation() {
        let rules = StatisticsEthicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("statistics_ethics"));
    }

    #[test]
    fn test_statisticsethicsrules_explain() {
        let rules = StatisticsEthicsRules::new();
        let e = rules.explain();
        assert!(e.contains("样本与收集"));
        assert!(e.contains("分析与呈现"));
        assert!(e.contains("结论与发布"));
    }
}
