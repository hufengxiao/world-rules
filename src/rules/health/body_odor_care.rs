//! 体味护理
//!
//! 体味过重的清洁习惯与日常护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BodyOdorCareRules,
    name: "体味护理",
    desc: "体味过重的清洁习惯与日常护理方法",
    origin: "中国",
    tags: ["皮肤", "护理", "生活"]
}

impl BodyOdorCareRules {
    /// 清洁习惯
    pub fn cleaning(&self) -> Vec<&'static str> {
        vec![
            "每日洗澡清洁",
            "重点清洗腋下腹股沟",
            "出汗后及时擦洗",
            "勤换贴身衣物",
        ]
    }

    /// 止汗抑菌
    pub fn control(&self) -> Vec<&'static str> {
        vec![
            "可用止汗剂减少出汗",
            "抑菌产品抑制细菌",
            "腋毛适度修剪",
            "保持腋下干燥",
        ]
    }

    /// 饮食调整
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "少食辛辣刺激食物",
            "减少洋葱大蒜摄入",
            "限制酒精与咖啡",
            "多饮水促进代谢",
        ]
    }

    /// 衣物选择
    pub fn clothing(&self) -> Vec<&'static str> {
        vec![
            "选棉麻透气面料",
            "避免紧身化纤衣物",
            "衣物及时清洗晾晒",
            "鞋子轮换通风",
        ]
    }
}

impl Rule for BodyOdorCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("body_odor_care")
    }

    fn explain(&self) -> String {
        format!(
            "【体味护理】\n{}",
            [
                format!(
                    "清洁习惯：\\n{}",
                    self.cleaning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "止汗抑菌：\\n{}",
                    self.control()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食调整：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "衣物选择：\\n{}",
                    self.clothing()
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
    fn test_bodyodorcarerules_basic() {
        let rules = BodyOdorCareRules::new();
        assert_eq!(rules.metadata().name, "体味护理");
        assert!(!rules.cleaning().is_empty());
        assert!(!rules.control().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.clothing().is_empty());
    }

    #[test]
    fn test_bodyodorcarerules_validation() {
        let rules = BodyOdorCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("body_odor_care"));
    }

    #[test]
    fn test_bodyodorcarerules_explain() {
        let rules = BodyOdorCareRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁习惯"));
        assert!(e.contains("止汗抑菌"));
        assert!(e.contains("饮食调整"));
    }
}
