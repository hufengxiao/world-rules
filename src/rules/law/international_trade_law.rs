//! 国际贸易法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: InternationalTradeLawRules, name: "国际贸易法", desc: "国际贸易法律规则", origin: "国际", tags: ["法律", "国际"] }
impl InternationalTradeLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["最惠国待遇", "争端解决"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["反倾销", "反补贴"]
    }
}
impl Rule for InternationalTradeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_trade_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "国际贸易法",
            &[("WTO", &self.section_0()), ("贸易救济", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalTradeLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
