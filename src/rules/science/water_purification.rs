//! 净水与水质
//!
//! 饮用水净化、烧开与水质安全常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WaterPurificationRules,
    name: "净水与水质",
    desc: "饮用水净化、烧开与水质安全常识",
    origin: "国际",
    tags: ["科学", "净水", "水质", "生活"]
}

impl WaterPurificationRules {
    /// 安全饮水
    pub fn safe(&self) -> Vec<&'static str> {
        vec!["白开水最放心", "烧开杀菌", "不喝生水", "水源有保障"]
    }

    /// 净水设备
    pub fn purifier(&self) -> Vec<&'static str> {
        vec!["滤水壶过滤杂质", "净水器去菌", "定期换滤芯", "按需选择"]
    }

    /// 水质判断
    pub fn quality(&self) -> Vec<&'static str> {
        vec!["观察色浊气味", "异常不饮用", "用烧开饮用", "不安可检测"]
    }

    /// 节约用水
    pub fn save(&self) -> Vec<&'static str> {
        vec!["用水勿浪费", "一水多用", "滴漏早修", "珍惜水资源"]
    }
}

impl Rule for WaterPurificationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("water_quality")
    }

    fn explain(&self) -> String {
        format!(
            "【净水与水质】\n{}",
            [
                format!(
                    "安全饮水：\\n{}",
                    self.safe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "净水设备：\\n{}",
                    self.purifier()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "水质判断：\\n{}",
                    self.quality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "节约用水：\\n{}",
                    self.save()
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
    fn test_waterpurificationrules_basic() {
        let rules = WaterPurificationRules::new();
        assert_eq!(rules.metadata().name, "净水与水质");
        assert!(!rules.safe().is_empty());
        assert!(!rules.purifier().is_empty());
        assert!(!rules.quality().is_empty());
        assert!(!rules.save().is_empty());
    }

    #[test]
    fn test_waterpurificationrules_validation() {
        let rules = WaterPurificationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("water_quality"));
    }

    #[test]
    fn test_waterpurificationrules_explain() {
        let rules = WaterPurificationRules::new();
        let e = rules.explain();
        assert!(e.contains("安全饮水"));
        assert!(e.contains("净水设备"));
        assert!(e.contains("水质判断"));
    }
}
