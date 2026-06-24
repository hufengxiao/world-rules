//! 植物学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BotanyDetailedRules, name: "植物学详细定律", desc: "植物学定律", origin: "国际", tags: ["科学", "生物"] }
impl BotanyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光合蒸腾"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["科属种"]
    }
}
impl Rule for BotanyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("botany_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "植物学详细定律",
            &[("生理", &self.section_0()), ("分类", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BotanyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
