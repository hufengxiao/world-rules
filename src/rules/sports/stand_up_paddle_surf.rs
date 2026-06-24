//! 站立桨板冲浪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: StandUpPaddleSurfRules, name: "站立桨板冲浪", desc: "站立桨板冲浪规则", origin: "美国", tags: ["体育", "水上"] }
impl StandUpPaddleSurfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["浪上技巧"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["板桨"]
    }
}
impl Rule for StandUpPaddleSurfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("stand_up_paddle_surf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "站立桨板冲浪",
            &[("比赛", &self.section_0()), ("装备", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = StandUpPaddleSurfRules::new();
        assert!(!r.explain().is_empty());
    }
}
