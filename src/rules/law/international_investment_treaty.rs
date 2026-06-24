//! 国际投资条约
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalInvestmentTreatyRules, name: "国际投资条约", desc: "国际投资条约规则", origin: "国际", tags: ["法律", "投资"] }
impl InternationalInvestmentTreatyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["双边投资条约"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["投资保护"]
    }
}
impl Rule for InternationalInvestmentTreatyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_investment_treaty")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际投资条约",
            &[("BIT", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalInvestmentTreatyRules::new();
        assert!(!r.explain().is_empty());
    }
}
