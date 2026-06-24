//! 民法典侵权详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodeTortRules, name: "民法典侵权详解", desc: "民法典侵权详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodeTortRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["过错责任"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["产品责任环境污染"]
    }
}
impl Rule for CivilCodeTortRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_tort")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典侵权详解",
            &[("一般", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodeTortRules::new();
        assert!(!r.explain().is_empty());
    }
}
