//! 电梯详细礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ElevatorDetailedRules,
    name: "电梯详细礼仪",
    desc: "电梯乘坐详细礼仪",
    origin: "国际",
    tags: ["社交", "公共"]
}

impl ElevatorDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["先下后上", "主动按键", "帮他人按楼层"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["让老人小孩先进", "帮按开门键", "电梯满时等下一趟"]
    }
}

impl Rule for ElevatorDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("elevator_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "电梯详细礼仪",
            &[("乘坐", &self.section_0()), ("礼让", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_elevator_detailed_rules() {
        let r = ElevatorDetailedRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
