//! 联合国宪章规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: UnCharterRules, name: "联合国宪章规则", desc: "联合国宪章规则", origin: "国际", tags: ["法律", "国际"] }
impl UnCharterRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["主权平等", "和平解决"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["安理会大会"]
    }
}
impl Rule for UnCharterRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("un_charter")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "联合国宪章规则",
            &[("原则", &self.section_0()), ("机构", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = UnCharterRules::new();
        assert!(!r.explain().is_empty());
    }
}
