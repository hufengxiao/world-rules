//! 精神病学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PsychiatryRules, name: "精神病学定律", desc: "精神病学定律", origin: "国际", tags: ["科学", "医学"] }
impl PsychiatryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["抑郁症精神分裂"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["药物心理"]
    }
}
impl Rule for PsychiatryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("psychiatry")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "精神病学定律",
            &[("疾病", &self.section_0()), ("治疗", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PsychiatryRules::new();
        assert!(!r.explain().is_empty());
    }
}
