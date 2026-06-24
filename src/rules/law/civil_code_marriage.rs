//! 民法典婚姻详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodeMarriageRules, name: "民法典婚姻详解", desc: "民法典婚姻详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodeMarriageRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["结婚条件"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["协议诉讼"]
    }
}
impl Rule for CivilCodeMarriageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_marriage")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典婚姻详解",
            &[("结婚", &self.section_0()), ("离婚", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodeMarriageRules::new();
        assert!(!r.explain().is_empty());
    }
}
