//! 莱德杯规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GolfRyderCupRules, name: "莱德杯规则", desc: "莱德杯高尔夫赛规则", origin: "国际", tags: ["体育", "球类"] }
impl GolfRyderCupRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["美欧对抗"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["四人两球", "四球赛"]
    }
}
impl Rule for GolfRyderCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("golf_ryder_cup")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "莱德杯规则",
            &[("赛制", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GolfRyderCupRules::new();
        assert!(!r.explain().is_empty());
    }
}
