//! 国际投资法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: InternationalInvestmentRules, name: "国际投资法", desc: "国际投资法律规则", origin: "国际", tags: ["法律", "国际"] }
impl InternationalInvestmentRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["公平公正待遇", "征收补偿"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["ICSID仲裁"]
    }
}
impl Rule for InternationalInvestmentRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_investment")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "国际投资法",
            &[("保护", &self.section_0()), ("争端", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalInvestmentRules::new();
        assert!(!r.explain().is_empty());
    }
}
