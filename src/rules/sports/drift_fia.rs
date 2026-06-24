//! 漂移赛FIA规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DriftFiaRules, name: "漂移赛FIA规则", desc: "FIA漂移赛规则", origin: "国际", tags: ["体育", "赛车"] }
impl DriftFiaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["角度速度线"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["单走追走"]
    }
}
impl Rule for DriftFiaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("drift_fia")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "漂移赛FIA规则",
            &[("评分", &self.section_0()), ("赛制", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DriftFiaRules::new();
        assert!(!r.explain().is_empty());
    }
}
