//! 便秘调理与预防
//!
//! 改善便秘的饮食纤维、水分与排便习惯调理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ConstipationReliefRules,
    name: "便秘调理与预防",
    desc: "改善便秘的饮食纤维、水分与排便习惯调理",
    origin: "医学",
    tags: ["健康", "便秘", "饮食", "排便"]
}

impl ConstipationReliefRules {
    /// 饮食纤维
    pub fn fiber(&self) -> Vec<&'static str> {
        vec![
            "多摄取膳食纤维",
            "食用全谷物蔬菜水果",
            "足量粗粮与豆类",
            "忌过度精细单一",
        ]
    }

    /// 水分活动
    pub fn water_activity(&self) -> Vec<&'static str> {
        vec![
            "每天充足饮水",
            "规律进行适度运动",
            "腹式深呼吸助肠动",
            "揉腹促进运化",
        ]
    }

    /// 排便习惯
    pub fn habit(&self) -> Vec<&'static str> {
        vec![
            "养成定时如厕习惯",
            "有便意不憋忍",
            "如厕不刷手机过度",
            "保持轻松不紧张",
        ]
    }

    /// 就医警示
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "长期便秘或突然改变就医",
            "伴出血腹痛消瘦注意",
            "老人儿童便秘谨慎去中",
            "不长期依赖泻药",
        ]
    }
}

impl Rule for ConstipationReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("constipation")
    }

    fn explain(&self) -> String {
        format!(
            "【便秘调理与预防】\n{}",
            [
                format!(
                    "饮食纤维：\\n{}",
                    self.fiber()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "水分活动：\\n{}",
                    self.water_activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "排便习惯：\\n{}",
                    self.habit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医警示：\\n{}",
                    self.seek_care()
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
    fn test_constipationreliefrules_basic() {
        let rules = ConstipationReliefRules::new();
        assert_eq!(rules.metadata().name, "便秘调理与预防");
        assert!(!rules.fiber().is_empty());
        assert!(!rules.water_activity().is_empty());
        assert!(!rules.habit().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_constipationreliefrules_validation() {
        let rules = ConstipationReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("constipation"));
    }

    #[test]
    fn test_constipationreliefrules_explain() {
        let rules = ConstipationReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食纤维"));
        assert!(e.contains("水分活动"));
        assert!(e.contains("排便习惯"));
    }
}
