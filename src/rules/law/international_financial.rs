//! 国际金融法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalFinancialRules, name: "国际金融法", desc: "国际金融法律规则", origin: "国际", tags: ["法律", "金融"] }
impl InternationalFinancialRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["银行监管"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["国际货币基金"]
    }
}
impl Rule for InternationalFinancialRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_financial")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际金融法",
            &[("巴塞尔", &self.section_0()), ("IMF", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalFinancialRules::new();
        assert!(!r.explain().is_empty());
    }
}
