//! 被害人权利法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: VictimRightsRules, name: "被害人权利法", desc: "被害人权利保障法", origin: "国际", tags: ["法律", "刑事"] }
impl VictimRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["知情参与"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["国家补偿"]
    }
}
impl Rule for VictimRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("victim_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "被害人权利法",
            &[("权利", &self.section_0()), ("赔偿", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VictimRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
