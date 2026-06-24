//! 生物化学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BiochemistryDetailedRules, name: "生物化学详细定律", desc: "生物化学定律", origin: "国际", tags: ["科学", "生物"] }
impl BiochemistryDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["糖酵解三羧酸循环"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["米氏方程"]
    }
}
impl Rule for BiochemistryDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("biochemistry_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生物化学详细定律",
            &[("代谢", &self.section_0()), ("酶", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BiochemistryDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
