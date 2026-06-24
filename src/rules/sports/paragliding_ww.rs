//! 滑翔伞世界规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ParaglidingWwRules, name: "滑翔伞世界规则", desc: "滑翔伞世界巡回赛", origin: "国际", tags: ["体育", "极限"] }
impl ParaglidingWwRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["定点越野竞速"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["气象装备"]
    }
}
impl Rule for ParaglidingWwRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("paragliding_ww")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "滑翔伞世界规则",
            &[("竞赛", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ParaglidingWwRules::new();
        assert!(!r.explain().is_empty());
    }
}
