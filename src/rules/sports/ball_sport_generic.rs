//! 球类通用规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BallSportGenericRules, name: "球类通用规则", desc: "球类运动通用规则", origin: "国际", tags: ["体育", "球类"] }
impl BallSportGenericRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["标准场地"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["裁判执法"]
    }
}
impl Rule for BallSportGenericRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ball_sport_generic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "球类通用规则",
            &[("场地", &self.section_0()), ("裁判", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BallSportGenericRules::new();
        assert!(!r.explain().is_empty());
    }
}
