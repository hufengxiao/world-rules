//! 民法典总则详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodeGeneralRules, name: "民法典总则详解", desc: "民法典总则详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodeGeneralRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自然人法人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["民事法律行为"]
    }
}
impl Rule for CivilCodeGeneralRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_general")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典总则详解",
            &[("主体", &self.section_0()), ("行为", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodeGeneralRules::new();
        assert!(!r.explain().is_empty());
    }
}
