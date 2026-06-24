//! 民法典物权详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodePropertyRules, name: "民法典物权详解", desc: "民法典物权详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodePropertyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国家集体私人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["土地承包"]
    }
}
impl Rule for CivilCodePropertyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_property")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典物权详解",
            &[("所有权", &self.section_0()), ("用益", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodePropertyRules::new();
        assert!(!r.explain().is_empty());
    }
}
