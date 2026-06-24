//! 机器人学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RoboticsDetailedRules, name: "机器人学详细定律", desc: "机器人学详细定律", origin: "国际", tags: ["科学", "工程"] }
impl RoboticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["运动学动力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["SLAM视觉"]
    }
}
impl Rule for RoboticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("robotics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "机器人学详细定律",
            &[("运动", &self.section_0()), ("感知", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RoboticsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
