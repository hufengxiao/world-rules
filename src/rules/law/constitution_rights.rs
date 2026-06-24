//! 宪法权利详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ConstitutionRightsRules, name: "宪法权利详解", desc: "宪法基本权利详解", origin: "中国", tags: ["法律", "宪法"] }
impl ConstitutionRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["平等自由社会权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["纳税服兵役"]
    }
}
impl Rule for ConstitutionRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("constitution_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "宪法权利详解",
            &[("权利", &self.section_0()), ("义务", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ConstitutionRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
