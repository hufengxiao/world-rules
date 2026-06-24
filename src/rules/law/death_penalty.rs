//! 死刑制度
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DeathPenaltyRules, name: "死刑制度", desc: "死刑法律制度", origin: "国际", tags: ["法律", "刑罚"] }
impl DeathPenaltyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["废除限制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["最严重罪行"]
    }
}
impl Rule for DeathPenaltyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("death_penalty")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "死刑制度",
            &[("趋势", &self.section_0()), ("标准", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DeathPenaltyRules::new();
        assert!(!r.explain().is_empty());
    }
}
