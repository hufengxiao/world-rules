//! 产后产妇照护
//!
//! 产妇产后的休息、营养与恢复照护

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PostpartumMotherCareRules,
    name: "产后产妇照护",
    desc: "产妇产后的休息、营养与恢复照护",
    origin: "医学",
    tags: ["健康", "产后", "产妇", "坐月子"]
}

impl PostpartumMotherCareRules {
    /// 充分休息
    pub fn rest(&self) -> Vec<&'static str> {
        vec!["产后多休息", "随宝宝作息", "不劳累操劳", "家人分担帮忙"]
    }

    /// 营养补充
    pub fn nutrition(&self) -> Vec<&'static str> {
        vec!["高蛋白易消化", "蔬果足量", "清淡不油腻", "多汤助乳"]
    }

    /// 伤口护理
    pub fn wound(&self) -> Vec<&'static str> {
        vec!["保持伤口清洁", "按医嘱复查", "恶露异常就医", "不坐浴泡盆"]
    }

    /// 情绪关照
    pub fn mood(&self) -> Vec<&'static str> {
        vec!["产后情绪波动正常", "家人多理解", "严重抑郁求助", "关爱产妇"]
    }
}

impl Rule for PostpartumMotherCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("postpartum")
    }

    fn explain(&self) -> String {
        format!(
            "【产后产妇照护】\n{}",
            [
                format!(
                    "充分休息：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营养补充：\\n{}",
                    self.nutrition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "伤口护理：\\n{}",
                    self.wound()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "情绪关照：\\n{}",
                    self.mood()
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
    fn test_postpartummothercarerules_basic() {
        let rules = PostpartumMotherCareRules::new();
        assert_eq!(rules.metadata().name, "产后产妇照护");
        assert!(!rules.rest().is_empty());
        assert!(!rules.nutrition().is_empty());
        assert!(!rules.wound().is_empty());
        assert!(!rules.mood().is_empty());
    }

    #[test]
    fn test_postpartummothercarerules_validation() {
        let rules = PostpartumMotherCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("postpartum"));
    }

    #[test]
    fn test_postpartummothercarerules_explain() {
        let rules = PostpartumMotherCareRules::new();
        let e = rules.explain();
        assert!(e.contains("充分休息"));
        assert!(e.contains("营养补充"));
        assert!(e.contains("伤口护理"));
    }
}
