//! 滑板奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SkateboardingOlympicRules, name: "滑板奥运规则", desc: "滑板奥运会规则", origin: "国际", tags: ["体育", "极限"] }
impl SkateboardingOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["道具技巧"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["高度流畅"]
    }
}
impl Rule for SkateboardingOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skateboarding_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "滑板奥运规则",
            &[("街式", &self.section_0()), ("碗池", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SkateboardingOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
