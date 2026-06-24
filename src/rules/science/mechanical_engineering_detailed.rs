//! 机械工程详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MechanicalEngineeringDetailedRules, name: "机械工程详细定律", desc: "机械工程定律", origin: "国际", tags: ["科学", "工程"] }
impl MechanicalEngineeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["强度刚度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["加工工艺"]
    }
}
impl Rule for MechanicalEngineeringDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("mechanical_engineering_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "机械工程详细定律",
            &[("设计", &self.section_0()), ("制造", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MechanicalEngineeringDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
