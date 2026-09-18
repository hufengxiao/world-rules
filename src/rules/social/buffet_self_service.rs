//! 自助餐举止
//!
//! 自助餐取餐多次、排队与避免浪费

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BuffetSelfServiceRules,
    name: "自助餐举止",
    desc: "自助餐取餐多次、排队与避免浪费",
    origin: "国际",
    tags: ["社交", "自助餐", "取餐", "礼仪"]
}

impl BuffetSelfServiceRules {
    /// 取餐有序
    pub fn queue(&self) -> Vec<&'static str> {
        vec!["排队依次取", "不随意插队", "看准再夹", "礼让老人小孩"]
    }

    /// 少取多次
    pub fn portion(&self) -> Vec<&'static str> {
        vec!["少取多次吃", "不一次堆满", "能吃多少拿多少", "避免浪费"]
    }

    /// 卫生细节
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec!["用公用夹子", "不边吃边取", "不翻捡食物", "保持洁净"]
    }

    /// 文明用餐
    pub fn demeanor(&self) -> Vec<&'static str> {
        vec!["夹到即吃", "不替他人代取", "遵守秩序", "愉快享用"]
    }
}

impl Rule for BuffetSelfServiceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("buffet")
    }

    fn explain(&self) -> String {
        format!(
            "【自助餐举止】\n{}",
            [
                format!(
                    "取餐有序：\\n{}",
                    self.queue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "少取多次：\\n{}",
                    self.portion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生细节：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "文明用餐：\\n{}",
                    self.demeanor()
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
    fn test_buffetselfservicerules_basic() {
        let rules = BuffetSelfServiceRules::new();
        assert_eq!(rules.metadata().name, "自助餐举止");
        assert!(!rules.queue().is_empty());
        assert!(!rules.portion().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.demeanor().is_empty());
    }

    #[test]
    fn test_buffetselfservicerules_validation() {
        let rules = BuffetSelfServiceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("buffet"));
    }

    #[test]
    fn test_buffetselfservicerules_explain() {
        let rules = BuffetSelfServiceRules::new();
        let e = rules.explain();
        assert!(e.contains("取餐有序"));
        assert!(e.contains("少取多次"));
        assert!(e.contains("卫生细节"));
    }
}
