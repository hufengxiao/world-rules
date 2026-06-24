//! 冰攀UIAA规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IceClimbingUiiaRules, name: "冰攀UIAA规则", desc: "冰攀国际规则", origin: "国际", tags: ["体育", "极限"] }
impl IceClimbingUiiaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["速度赛难度赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["冰镐冰爪"]
    }
}
impl Rule for IceClimbingUiiaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ice_climbing_uiia")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "冰攀UIAA规则",
            &[("比赛", &self.section_0()), ("装备", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IceClimbingUiiaRules::new();
        assert!(!r.explain().is_empty());
    }
}
