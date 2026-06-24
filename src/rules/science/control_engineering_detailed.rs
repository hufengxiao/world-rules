//! 控制工程详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ControlEngineeringDetailedRules, name: "控制工程详细定律", desc: "控制工程定律", origin: "国际", tags: ["科学", "工程"] }
impl ControlEngineeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["PID控制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["状态空间"]
    }
}
impl Rule for ControlEngineeringDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("control_engineering_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "控制工程详细定律",
            &[("经典", &self.section_0()), ("现代", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ControlEngineeringDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
