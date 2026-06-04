//! 机器人学理论
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: RoboticsTheoryRules, name: "机器人学理论", desc: "机器人学理论定律", origin: "国际", tags: ["科学", "工程"] }
impl RoboticsTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["正运动学", "轨迹规划"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["SLAM", "力反馈"]
    }
}
impl Rule for RoboticsTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("robotics_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "机器人学理论",
            &[("运动", &self.section_0()), ("感知", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RoboticsTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
