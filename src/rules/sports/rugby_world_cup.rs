//! 橄榄球世界杯规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RugbyWorldCupRules, name: "橄榄球世界杯规则", desc: "橄榄球世界杯规则", origin: "国际", tags: ["体育", "球类"] }
impl RugbyWorldCupRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["小组赛淘汰赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["达阵规则"]
    }
}
impl Rule for RugbyWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rugby_world_cup")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "橄榄球世界杯规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RugbyWorldCupRules::new();
        assert!(!r.explain().is_empty());
    }
}
