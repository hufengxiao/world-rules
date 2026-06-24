//! 公司法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CompanyLawDetailedRules, name: "公司法详解", desc: "公司法详解", origin: "中国", tags: ["法律", "公司"] }
impl CompanyLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["有限公司股份公司"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["股东会董事会"]
    }
}
impl Rule for CompanyLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("company_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "公司法详解",
            &[("设立", &self.section_0()), ("治理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CompanyLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
