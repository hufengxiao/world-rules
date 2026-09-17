//! 用餐座次
//!
//! 中式宴席主宾、长幼的座次安排礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DinnerSeatingOrderRules,
    name: "用餐座次",
    desc: "中式宴席主宾、长幼的座次安排礼仪",
    origin: "中国",
    tags: ["社交", "座次", "宴席", "礼仪"]
}

impl DinnerSeatingOrderRules {
    /// 主宾之位
    pub fn honor(&self) -> Vec<&'static str> {
        vec![
            "主位面向门口方向",
            "主宾坐主位旁",
            "次宾相对方位",
            "主人主陪全场",
        ]
    }

    /// 长幼有序
    pub fn order(&self) -> Vec<&'static str> {
        vec!["长辈坐上首", "晚辈主动让座", "夫妻可相邻", "身份相近相邻"]
    }

    /// 入座礼让
    pub fn politeness(&self) -> Vec<&'static str> {
        vec!["请长辈先坐", "不抢位不占", "主人让座再坐", "起座轻扶椅"]
    }

    /// 圆桌规矩
    pub fn round(&self) -> Vec<&'static str> {
        vec![
            "主位在背靠墙这端",
            "左右两侧有序",
            "转盘先供客",
            "转动转盘礼貌",
        ]
    }
}

impl Rule for DinnerSeatingOrderRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("seating_order")
    }

    fn explain(&self) -> String {
        format!(
            "【用餐座次】\n{}",
            [
                format!(
                    "主宾之位：\\n{}",
                    self.honor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长幼有序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "入座礼让：\\n{}",
                    self.politeness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "圆桌规矩：\\n{}",
                    self.round()
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
    fn test_dinnerseatingorderrules_basic() {
        let rules = DinnerSeatingOrderRules::new();
        assert_eq!(rules.metadata().name, "用餐座次");
        assert!(!rules.honor().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.politeness().is_empty());
        assert!(!rules.round().is_empty());
    }

    #[test]
    fn test_dinnerseatingorderrules_validation() {
        let rules = DinnerSeatingOrderRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("seating_order"));
    }

    #[test]
    fn test_dinnerseatingorderrules_explain() {
        let rules = DinnerSeatingOrderRules::new();
        let e = rules.explain();
        assert!(e.contains("主宾之位"));
        assert!(e.contains("长幼有序"));
        assert!(e.contains("入座礼让"));
    }
}
