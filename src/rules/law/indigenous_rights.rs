//! 原住民权利法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IndigenousRightsRules, name: "原住民权利法", desc: "原住民权利保障法", origin: "国际", tags: ["法律", "原住民"] }
impl IndigenousRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自决权土地权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["文化遗产"]
    }
}
impl Rule for IndigenousRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("indigenous_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "原住民权利法",
            &[("权利", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IndigenousRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
