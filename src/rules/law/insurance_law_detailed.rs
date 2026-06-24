//! 保险法详解2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InsuranceLawDetailedRules, name: "保险法详解2", desc: "保险法详解2", origin: "中国", tags: ["法律", "保险"] }
impl InsuranceLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["投保理赔"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["偿付能力"]
    }
}
impl Rule for InsuranceLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("insurance_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "保险法详解2",
            &[("合同", &self.section_0()), ("监管", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InsuranceLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
