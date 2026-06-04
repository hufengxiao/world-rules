//! 金融经济学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: FinancialEconomicsRules, name: "金融经济学定律", desc: "金融经济学定律", origin: "国际", tags: ["科学", "经济"] }
impl FinancialEconomicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["CAPM模型", "期权定价"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有效市场假说", "投资组合"]
    }
}
impl Rule for FinancialEconomicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("financial_economics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "金融经济学定律",
            &[("定价", &self.section_0()), ("风险", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FinancialEconomicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
