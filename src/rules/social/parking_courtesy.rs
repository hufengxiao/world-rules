//! 停车礼让规范
//!
//! 停车入位、让行与公共停车秩序礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ParkingCourtesyRules,
    name: "停车礼让规范",
    desc: "停车入位、让行与公共停车秩序礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "停车", "秩序"]
}

impl ParkingCourtesyRules {
    /// 规范停车
    pub fn park(&self) -> Vec<&'static str> {
        vec![
            "停入划定车位线内",
            "不占他人专属车位",
            "不挡通道出入",
            "按方向有序停放",
        ]
    }

    /// 让行礼让
    pub fn yielding(&self) -> Vec<&'static str> {
        vec![
            "进出车位注意行人与车辆",
            "到点让出车位",
            "不抢他人车位",
            "狭小空间友善协商",
        ]
    }

    /// 特殊车辆
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "不占无障碍车位",
            "为应急车辆让路",
            "孩童老人车位礼让",
            "依规使用专用位",
        ]
    }

    /// 公共秩序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "不乱停乱放",
            "遵守场所停车规定",
            "不与守规争吵",
            "共同维护秩序",
        ]
    }
}

impl Rule for ParkingCourtesyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("parking")
    }

    fn explain(&self) -> String {
        format!(
            "【停车礼让规范】\n{}",
            [
                format!(
                    "规范停车：\\n{}",
                    self.park()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "让行礼让：\\n{}",
                    self.yielding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊车辆：\\n{}",
                    self.special()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共秩序：\\n{}",
                    self.order()
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
    fn test_parkingcourtesyrules_basic() {
        let rules = ParkingCourtesyRules::new();
        assert_eq!(rules.metadata().name, "停车礼让规范");
        assert!(!rules.park().is_empty());
        assert!(!rules.yielding().is_empty());
        assert!(!rules.special().is_empty());
        assert!(!rules.order().is_empty());
    }

    #[test]
    fn test_parkingcourtesyrules_validation() {
        let rules = ParkingCourtesyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("parking"));
    }

    #[test]
    fn test_parkingcourtesyrules_explain() {
        let rules = ParkingCourtesyRules::new();
        let e = rules.explain();
        assert!(e.contains("规范停车"));
        assert!(e.contains("让行礼让"));
        assert!(e.contains("特殊车辆"));
    }
}
