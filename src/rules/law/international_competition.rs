//! 国际竞争法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalCompetitionRules, name: "国际竞争法", desc: "国际竞争法律规则", origin: "国际", tags: ["法律", "竞争"] }
impl InternationalCompetitionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["跨国合并"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["国际卡特尔"]
    }
}
impl Rule for InternationalCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_competition")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际竞争法",
            &[("合并", &self.section_0()), ("卡特尔", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalCompetitionRules::new();
        assert!(!r.explain().is_empty());
    }
}
