//! 热传递的三种方式
//!
//! 热传导、对流与辐射三种传热的日常原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HeatTransferThreeWaysRules,
    name: "热传递的三种方式",
    desc: "热传导、对流与辐射三种传热的日常原理",
    origin: "中国",
    tags: ["科学", "传热", "物理", "热"]
}

impl HeatTransferThreeWaysRules {
    /// 热传导
    pub fn conduction(&self) -> Vec<&'static str> {
        vec!["接触传热", "金属导热快", "厚手套隔热", "热由高温到低温"]
    }

    /// 热对流
    pub fn convection(&self) -> Vec<&'static str> {
        vec![
            "液体气体流动传热",
            "热水上升冷水降",
            "锅煮水翻腾",
            "空调气流循环",
        ]
    }

    /// 热辐射
    pub fn radiation(&self) -> Vec<&'static str> {
        vec![
            "不需介质传热",
            "太阳辐射可传",
            "火光红外辐射",
            "真空中也能传导",
        ]
    }

    /// 生活应用
    pub fn apply(&self) -> Vec<&'static str> {
        vec![
            "暖手宝贴身传导",
            "暖炉辐射取暖",
            "通风对流降温",
            "三层方式并用",
        ]
    }
}

impl Rule for HeatTransferThreeWaysRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("heat_transfer")
    }

    fn explain(&self) -> String {
        format!(
            "【热传递的三种方式】\n{}",
            [
                format!(
                    "热传导：\\n{}",
                    self.conduction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "热对流：\\n{}",
                    self.convection()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "热辐射：\\n{}",
                    self.radiation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活应用：\\n{}",
                    self.apply()
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
    fn test_heattransferthreewaysrules_basic() {
        let rules = HeatTransferThreeWaysRules::new();
        assert_eq!(rules.metadata().name, "热传递的三种方式");
        assert!(!rules.conduction().is_empty());
        assert!(!rules.convection().is_empty());
        assert!(!rules.radiation().is_empty());
        assert!(!rules.apply().is_empty());
    }

    #[test]
    fn test_heattransferthreewaysrules_validation() {
        let rules = HeatTransferThreeWaysRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("heat_transfer"));
    }

    #[test]
    fn test_heattransferthreewaysrules_explain() {
        let rules = HeatTransferThreeWaysRules::new();
        let e = rules.explain();
        assert!(e.contains("热传导"));
        assert!(e.contains("热对流"));
        assert!(e.contains("热辐射"));
    }
}
