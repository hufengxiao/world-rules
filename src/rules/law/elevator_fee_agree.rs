//! 电梯费与维护
//!
//! 电梯维护、使用分摊与费用权利义务

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElevatorFeeAgreeRules,
    name: "电梯费与维护",
    desc: "电梯维护、使用分摊与费用权利义务",
    origin: "中国",
    tags: ["法律", "电梯", "费用", "维护"]
}

impl ElevatorFeeAgreeRules {
    /// 维保义务
    pub fn maintain(&self) -> Vec<&'static str> {
        vec!["电梯定期检修", "物业负责维护", "按时保养", "安全运行"]
    }

    /// 费用分摊
    pub fn share(&self) -> Vec<&'static str> {
        vec!["费用依规分摊", "业主合理承担", "明示账目", "公开透明"]
    }

    /// 文明使用
    pub fn usage(&self) -> Vec<&'static str> {
        vec!["乘梯不超载", "不倚不蹦跳", "礼让老幼", "安全乘用"]
    }

    /// 故障报修
    pub fn repair(&self) -> Vec<&'static str> {
        vec!["故障停梯立报修", "困梯按铃求助", "不私自维修", "及时处理"]
    }
}

impl Rule for ElevatorFeeAgreeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("elevator_fee")
    }

    fn explain(&self) -> String {
        format!(
            "【电梯费与维护】\n{}",
            [
                format!(
                    "维保义务：\\n{}",
                    self.maintain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "费用分摊：\\n{}",
                    self.share()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "文明使用：\\n{}",
                    self.usage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "故障报修：\\n{}",
                    self.repair()
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
    fn test_elevatorfeeagreerules_basic() {
        let rules = ElevatorFeeAgreeRules::new();
        assert_eq!(rules.metadata().name, "电梯费与维护");
        assert!(!rules.maintain().is_empty());
        assert!(!rules.share().is_empty());
        assert!(!rules.usage().is_empty());
        assert!(!rules.repair().is_empty());
    }

    #[test]
    fn test_elevatorfeeagreerules_validation() {
        let rules = ElevatorFeeAgreeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("elevator_fee"));
    }

    #[test]
    fn test_elevatorfeeagreerules_explain() {
        let rules = ElevatorFeeAgreeRules::new();
        let e = rules.explain();
        assert!(e.contains("维保义务"));
        assert!(e.contains("费用分摊"));
        assert!(e.contains("文明使用"));
    }
}
