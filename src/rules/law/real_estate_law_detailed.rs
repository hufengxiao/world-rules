//! 房地产法详解2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RealEstateLawDetailedRules, name: "房地产法详解2", desc: "房地产法详解2", origin: "中国", tags: ["法律", "房产"] }
impl RealEstateLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["预售许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["过户登记"]
    }
}
impl Rule for RealEstateLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("real_estate_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "房地产法详解2",
            &[("开发", &self.section_0()), ("交易", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RealEstateLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
