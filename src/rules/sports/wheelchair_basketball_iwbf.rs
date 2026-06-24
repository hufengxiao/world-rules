//! 轮椅篮球IWBF
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WheelchairBasketballIwbfRules, name: "轮椅篮球IWBF", desc: "IWBF轮椅篮球规则", origin: "国际", tags: ["体育", "残疾人"] }
impl WheelchairBasketballIwbfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["1.0到4.5分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["总分限制"]
    }
}
impl Rule for WheelchairBasketballIwbfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_basketball_iwbf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "轮椅篮球IWBF",
            &[("分级", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WheelchairBasketballIwbfRules::new();
        assert!(!r.explain().is_empty());
    }
}
