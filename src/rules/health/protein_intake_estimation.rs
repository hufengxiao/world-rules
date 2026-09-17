//! 蛋白质摄入
//!
//! 每日蛋白质需要、来源与适量摄入

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProteinIntakeEstimationRules,
    name: "蛋白质摄入",
    desc: "每日蛋白质需要、来源与适量摄入",
    origin: "营养学",
    tags: ["健康", "蛋白质", "营养"]
}

impl ProteinIntakeEstimationRules {
    /// 生理需要
    pub fn need(&self) -> Vec<&'static str> {
        vec![
            "优质蛋白助修复",
            "支持肌肉免疫",
            "按体重估算",
            "成人适量摄入",
        ]
    }

    /// 食物来源
    pub fn source(&self) -> Vec<&'static str> {
        vec!["鱼禽蛋奶富含", "豆类植物蛋白", "瘦肉适量", "搭配互补"]
    }

    /// 不过量
    pub fn moderate(&self) -> Vec<&'static str> {
        vec!["不必大量补蛋白", "有过量负担", "分配各餐", "按需摄入"]
    }

    /// 特殊情况
    pub fn special(&self) -> Vec<&'static str> {
        vec!["病患健身按需增", "肝肾问题问医", "儿童老人关注", "均衡为准"]
    }
}

impl Rule for ProteinIntakeEstimationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("protein")
    }

    fn explain(&self) -> String {
        format!(
            "【蛋白质摄入】\n{}",
            [
                format!(
                    "生理需要：\\n{}",
                    self.need()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "食物来源：\\n{}",
                    self.source()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "不过量：\\n{}",
                    self.moderate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊情况：\\n{}",
                    self.special()
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
    fn test_proteinintakeestimationrules_basic() {
        let rules = ProteinIntakeEstimationRules::new();
        assert_eq!(rules.metadata().name, "蛋白质摄入");
        assert!(!rules.need().is_empty());
        assert!(!rules.source().is_empty());
        assert!(!rules.moderate().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_proteinintakeestimationrules_validation() {
        let rules = ProteinIntakeEstimationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("protein"));
    }

    #[test]
    fn test_proteinintakeestimationrules_explain() {
        let rules = ProteinIntakeEstimationRules::new();
        let e = rules.explain();
        assert!(e.contains("生理需要"));
        assert!(e.contains("食物来源"));
        assert!(e.contains("不过量"));
    }
}
