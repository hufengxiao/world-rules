//! 公司治理法规
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CorporateGovernanceRules, name: "公司治理法规", desc: "公司治理法律规则", origin: "中国", tags: ["法律", "公司"] }
impl CorporateGovernanceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["决议程序", "股东权利"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["董事义务", "独立董事"]
    }
}
impl Rule for CorporateGovernanceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("corporate_governance")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "公司治理法规",
            &[("股东会", &self.section_0()), ("董事会", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CorporateGovernanceRules::new();
        assert!(!r.explain().is_empty());
    }
}
