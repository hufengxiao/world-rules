//! 老年人权利法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ElderlyRightsRules, name: "老年人权利法", desc: "老年人权利保障法", origin: "国际", tags: ["法律", "老年"] }
impl ElderlyRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["尊严照料"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["社会保障"]
    }
}
impl Rule for ElderlyRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("elderly_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "老年人权利法",
            &[("权利", &self.section_0()), ("保障", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ElderlyRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
