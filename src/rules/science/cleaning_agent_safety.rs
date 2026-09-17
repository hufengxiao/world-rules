//! 清洁剂安全常识
//!
//! 清洁剂的分类、混合风险与安全使用常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CleaningAgentSafetyRules,
    name: "清洁剂安全常识",
    desc: "清洁剂的分类、混合风险与安全使用常识",
    origin: "国际",
    tags: ["科学", "清洁剂", "安全", "化学"]
}

impl CleaningAgentSafetyRules {
    /// 分类认识
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "洗涤剂去污日常",
            "消毒剂针对病原",
            "强酸碱有腐蚀性",
            "挥发溶剂保持通风",
            "存放于儿童不可及处",
        ]
    }

    /// 禁止混用
    pub fn donot_mix(&self) -> Vec<&'static str> {
        vec![
            "酸性与含氯不混用",
            "混用可能产生毒气",
            "按说明单独使用",
            "不丢弃产品标签",
        ]
    }

    /// 安全使用
    pub fn usage(&self) -> Vec<&'static str> {
        vec!["佩戴手套护目", "保持通风开窗", "按说明稀释", "远离食品儿童"]
    }

    /// 应急处理
    pub fn emergency(&self) -> Vec<&'static str> {
        vec![
            "溅入眼睛立即清洗就医",
            "误食勿催吐就医",
            "皮肤接触冲水",
            "有毒烟立即通风撤离",
        ]
    }
}

impl Rule for CleaningAgentSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("cleaning_safety")
    }

    fn explain(&self) -> String {
        format!(
            "【清洁剂安全常识】\n{}",
            [
                format!(
                    "分类认识：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "禁止混用：\\n{}",
                    self.donot_mix()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全使用：\\n{}",
                    self.usage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应急处理：\\n{}",
                    self.emergency()
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
    fn test_cleaningagentsafetyrules_basic() {
        let rules = CleaningAgentSafetyRules::new();
        assert_eq!(rules.metadata().name, "清洁剂安全常识");
        assert!(!rules.types().is_empty());
        assert!(!rules.donot_mix().is_empty());
        assert!(!rules.usage().is_empty());
        assert!(!rules.emergency().is_empty());
    }

    #[test]
    fn test_cleaningagentsafetyrules_validation() {
        let rules = CleaningAgentSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("cleaning_safety"));
    }

    #[test]
    fn test_cleaningagentsafetyrules_explain() {
        let rules = CleaningAgentSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("分类认识"));
        assert!(e.contains("禁止混用"));
        assert!(e.contains("安全使用"));
    }
}
