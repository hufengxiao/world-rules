//! 航空运动规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AirSportsRules, name: "航空运动规则", desc: "航空运动竞赛规则", origin: "国际", tags: ["体育", "航空"] }
impl AirSportsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["滑翔跳伞"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["飞行规则"]
    }
}
impl Rule for AirSportsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("air_sports")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "航空运动规则",
            &[("类型", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AirSportsRules::new();
        assert!(!r.explain().is_empty());
    }
}
