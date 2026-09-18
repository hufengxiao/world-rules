//! 跳绳健身玩法
//!
//! 跳绳花样、计时与多人跳绳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JumpRopeSkippingRules,
    name: "跳绳健身玩法",
    desc: "跳绳花样、计时与多人跳绳规则",
    origin: "国际",
    tags: ["游戏", "跳绳", "健身"]
}

impl JumpRopeSkippingRules {
    /// 基本跳法
    pub fn basic(&self) -> Vec<&'static str> {
        vec!["手握绳两端", "摇绳过身", "双脚合跳", "节奏均匀"]
    }

    /// 花样进阶
    pub fn style(&self) -> Vec<&'static str> {
        vec!["单脚交替换跳", "双摇更快", "交叉跳花样", "循序渐进"]
    }

    /// 多人跳绳
    pub fn group(&self) -> Vec<&'static str> {
        vec!["两人摇长绳", "一人或多人跳", "依次进出", "配合节奏"]
    }

    /// 运动安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["软地面跳", "穿适合鞋", "活动手腕脚踝", "量力而为"]
    }
}

impl Rule for JumpRopeSkippingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("jump_rope")
    }

    fn explain(&self) -> String {
        format!(
            "【跳绳健身玩法】\n{}",
            [
                format!(
                    "基本跳法：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "花样进阶：\\n{}",
                    self.style()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "多人跳绳：\\n{}",
                    self.group()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动安全：\\n{}",
                    self.safety()
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
    fn test_jumpropeskippingrules_basic() {
        let rules = JumpRopeSkippingRules::new();
        assert_eq!(rules.metadata().name, "跳绳健身玩法");
        assert!(!rules.basic().is_empty());
        assert!(!rules.style().is_empty());
        assert!(!rules.group().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_jumpropeskippingrules_validation() {
        let rules = JumpRopeSkippingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("jump_rope"));
    }

    #[test]
    fn test_jumpropeskippingrules_explain() {
        let rules = JumpRopeSkippingRules::new();
        let e = rules.explain();
        assert!(e.contains("基本跳法"));
        assert!(e.contains("花样进阶"));
        assert!(e.contains("多人跳绳"));
    }
}
