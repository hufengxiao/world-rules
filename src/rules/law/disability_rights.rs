//! 残疾人权利法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DisabilityRightsRules, name: "残疾人权利法", desc: "残疾人权利保障法", origin: "国际", tags: ["法律", "残疾人"] }
impl DisabilityRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["平等参与"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["合理便利"]
    }
}
impl Rule for DisabilityRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("disability_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "残疾人权利法",
            &[("权利", &self.section_0()), ("合理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DisabilityRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
