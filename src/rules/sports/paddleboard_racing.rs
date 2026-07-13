//! 桨板竞速规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PaddleboardRacingRules, name: "竞技桨板规则", desc: "桨板竞速规则", origin: "国际", tags: ["体育", "水上"] }
impl PaddleboardRacingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["长距离短距离"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["板桨脚绳"]
    }
}
impl Rule for PaddleboardRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("paddleboard_racing")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "桨板竞速规则",
            &[("比赛", &self.section_0()), ("装备", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PaddleboardRacingRules::new();
        assert!(!r.explain().is_empty());
    }
}
