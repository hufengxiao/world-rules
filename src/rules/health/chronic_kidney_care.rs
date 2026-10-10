//! 慢性肾病管理
//!
//! 慢性肾脏病的饮食与生活管理要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChronicKidneyCareRules,
    name: "慢性肾病管理",
    desc: "慢性肾脏病的饮食与生活管理要点",
    origin: "中国",
    tags: ["慢性病", "肾脏", "管理"]
}

impl ChronicKidneyCareRules {
    /// 饮食控制
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "控制蛋白质摄入量",
            "限盐减轻肾脏负担",
            "控制钾磷摄入",
            "遵医嘱调整饮水",
        ]
    }

    /// 血压血糖
    pub fn control(&self) -> Vec<&'static str> {
        vec!["严格控血压", "控制血糖防加重", "遵医嘱服药", "定期监测指标"]
    }

    /// 生活管理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec!["避免劳累与感染", "慎用伤肾药物", "适度运动", "规律作息"]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "定期查肾功能",
            "尿量异常及时就医",
            "水肿加重应就诊",
            "勿滥用偏方药物",
        ]
    }
}

impl Rule for ChronicKidneyCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("chronic_kidney_care")
    }

    fn explain(&self) -> String {
        format!(
            "【慢性肾病管理】\n{}",
            [
                format!(
                    "饮食控制：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "血压血糖：\\n{}",
                    self.control()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活管理：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_chronickidneycarerules_basic() {
        let rules = ChronicKidneyCareRules::new();
        assert_eq!(rules.metadata().name, "慢性肾病管理");
        assert!(!rules.diet().is_empty());
        assert!(!rules.control().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_chronickidneycarerules_validation() {
        let rules = ChronicKidneyCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("chronic_kidney_care")
        );
    }

    #[test]
    fn test_chronickidneycarerules_explain() {
        let rules = ChronicKidneyCareRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食控制"));
        assert!(e.contains("血压血糖"));
        assert!(e.contains("生活管理"));
    }
}
