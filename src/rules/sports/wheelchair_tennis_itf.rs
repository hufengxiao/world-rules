//! 轮椅网球ITF
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WheelchairTennisItfRules, name: "轮椅网球ITF", desc: "ITF轮椅网球规则", origin: "国际", tags: ["体育", "残疾人"] }
impl WheelchairTennisItfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["两跳规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["单打双打"]
    }
}
impl Rule for WheelchairTennisItfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_tennis_itf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "轮椅网球ITF",
            &[("规则", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WheelchairTennisItfRules::new();
        assert!(!r.explain().is_empty());
    }
}
