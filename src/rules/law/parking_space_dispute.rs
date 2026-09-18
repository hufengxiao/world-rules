//! 停车位纠纷
//!
//! 车位占用、堵塞与物业协调处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ParkingSpaceDisputeRules,
    name: "停车位纠纷",
    desc: "车位占用、堵塞与物业协调处理",
    origin: "中国",
    tags: ["法律", "停车", "车位", "纠纷"]
}

impl ParkingSpaceDisputeRules {
    /// 规范停车
    pub fn comply(&self) -> Vec<&'static str> {
        vec!["停入规定车位", "不占他人位", "不堵通道", "线内停放"]
    }

    /// 被占处理
    pub fn occupy(&self) -> Vec<&'static str> {
        vec!["找物业联系车主", "打电话移车", "不作报复", "理性解决"]
    }

    /// 堵塞处置
    pub fn block(&self) -> Vec<&'static str> {
        vec!["堵车出入口立移", "打移车电话", "记录证据", "紧急可报警"]
    }

    /// 协商解决
    pub fn resolve(&self) -> Vec<&'static str> {
        vec!["车位争议协商", "依产权合同", "调解或诉讼", "依法而定"]
    }
}

impl Rule for ParkingSpaceDisputeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("parking")
    }

    fn explain(&self) -> String {
        format!(
            "【停车位纠纷】\n{}",
            [
                format!(
                    "规范停车：\\n{}",
                    self.comply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "被占处理：\\n{}",
                    self.occupy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "堵塞处置：\\n{}",
                    self.block()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "协商解决：\\n{}",
                    self.resolve()
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
    fn test_parkingspacedisputerules_basic() {
        let rules = ParkingSpaceDisputeRules::new();
        assert_eq!(rules.metadata().name, "停车位纠纷");
        assert!(!rules.comply().is_empty());
        assert!(!rules.occupy().is_empty());
        assert!(!rules.block().is_empty());
        assert!(!rules.resolve().is_empty());
    }

    #[test]
    fn test_parkingspacedisputerules_validation() {
        let rules = ParkingSpaceDisputeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("parking"));
    }

    #[test]
    fn test_parkingspacedisputerules_explain() {
        let rules = ParkingSpaceDisputeRules::new();
        let e = rules.explain();
        assert!(e.contains("规范停车"));
        assert!(e.contains("被占处理"));
        assert!(e.contains("堵塞处置"));
    }
}
