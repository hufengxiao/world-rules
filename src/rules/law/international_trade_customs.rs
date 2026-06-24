//! 国际贸易海关法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalTradeCustomsRules, name: "国际贸易海关法", desc: "国际贸易海关规则", origin: "国际", tags: ["法律", "海关"] }
impl InternationalTradeCustomsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["原产地规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["海关估价"]
    }
}
impl Rule for InternationalTradeCustomsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_trade_customs")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际贸易海关法",
            &[("原产", &self.section_0()), ("估价", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalTradeCustomsRules::new();
        assert!(!r.explain().is_empty());
    }
}
