//! 住房权法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RightToHousingRules, name: "住房权法", desc: "住房权法律规则", origin: "国际", tags: ["法律", "住房"] }
impl RightToHousingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["适足住房"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["住房保障"]
    }
}
impl Rule for RightToHousingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("right_to_housing")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "住房权法",
            &[("权利", &self.section_0()), ("保障", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RightToHousingRules::new();
        assert!(!r.explain().is_empty());
    }
}
