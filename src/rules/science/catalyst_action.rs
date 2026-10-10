//! 催化剂作用
//!
//! 催化剂降低反应活化能加快反应速率的原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CatalystActionRules,
    name: "催化剂作用",
    desc: "催化剂降低反应活化能加快反应速率的原理",
    origin: "科学常识",
    tags: ["化学", "反应", "催化"]
}

impl CatalystActionRules {
    /// 作用原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "降低反应活化能",
            "提供替代反应路径",
            "加快反应速率",
            "自身质量化学性质不变",
        ]
    }

    /// 选择特性
    pub fn selectivity(&self) -> Vec<&'static str> {
        vec![
            "催化剂具有选择性",
            "特定反应用特定催化剂",
            "不改变反应方向",
            "不改变平衡常数",
        ]
    }

    /// 酶催化
    pub fn enzyme(&self) -> Vec<&'static str> {
        vec![
            "酶是生物催化剂",
            "高效专一性极强",
            "条件温和效率高",
            "广泛用于生命活动",
        ]
    }

    /// 工业应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "合成氨用铁催化剂",
            "汽车尾气净化催化",
            "石油裂化催化",
            "提高生产效率",
        ]
    }
}

impl Rule for CatalystActionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("catalyst_action")
    }

    fn explain(&self) -> String {
        format!(
            "【催化剂作用】\n{}",
            [
                format!(
                    "作用原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "选择特性：\\n{}",
                    self.selectivity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "酶催化：\\n{}",
                    self.enzyme()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "工业应用：\\n{}",
                    self.applications()
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
    fn test_catalystactionrules_basic() {
        let rules = CatalystActionRules::new();
        assert_eq!(rules.metadata().name, "催化剂作用");
        assert!(!rules.principle().is_empty());
        assert!(!rules.selectivity().is_empty());
        assert!(!rules.enzyme().is_empty());
        assert!(!rules.applications().is_empty());
    }

    #[test]
    fn test_catalystactionrules_validation() {
        let rules = CatalystActionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("catalyst_action"));
    }

    #[test]
    fn test_catalystactionrules_explain() {
        let rules = CatalystActionRules::new();
        let e = rules.explain();
        assert!(e.contains("作用原理"));
        assert!(e.contains("选择特性"));
        assert!(e.contains("酶催化"));
    }
}
