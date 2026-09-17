//! 高铁乘车礼仪
//!
//! 高铁进站、乘车与座位的规则礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HighSpeedRailMannerRules,
    name: "高铁乘车礼仪",
    desc: "高铁进站、乘车与座位的规则礼仪",
    origin: "中国",
    tags: ["社交", "高铁", "乘车", "礼仪"]
}

impl HighSpeedRailMannerRules {
    /// 检票进站
    pub fn boarding(&self) -> Vec<&'static str> {
        vec!["提前取票过检", "按标识进站台", "排队有序上车", "行李先放好"]
    }

    /// 座内礼仪
    pub fn seat(&self) -> Vec<&'static str> {
        vec!["对号入座", "合理调靠背", "放置行李贴稳", "不占他人位"]
    }

    /// 安静乘车
    pub fn quiet(&self) -> Vec<&'static str> {
        vec!["不大声喧哗", "打电话低声", "耳机不外放", "保持安静"]
    }

    /// 下车有序
    pub fn alight(&self) -> Vec<&'static str> {
        vec!["到站提前准备", "依序下车", "不拥挤", "垃圾带走"]
    }
}

impl Rule for HighSpeedRailMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("hsr")
    }

    fn explain(&self) -> String {
        format!(
            "【高铁乘车礼仪】\n{}",
            [
                format!(
                    "检票进站：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "座内礼仪：\\n{}",
                    self.seat()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安静乘车：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "下车有序：\\n{}",
                    self.alight()
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
    fn test_highspeedrailmannerrules_basic() {
        let rules = HighSpeedRailMannerRules::new();
        assert_eq!(rules.metadata().name, "高铁乘车礼仪");
        assert!(!rules.boarding().is_empty());
        assert!(!rules.seat().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.alight().is_empty());
    }

    #[test]
    fn test_highspeedrailmannerrules_validation() {
        let rules = HighSpeedRailMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("hsr"));
    }

    #[test]
    fn test_highspeedrailmannerrules_explain() {
        let rules = HighSpeedRailMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("检票进站"));
        assert!(e.contains("座内礼仪"));
        assert!(e.contains("安静乘车"));
    }
}
