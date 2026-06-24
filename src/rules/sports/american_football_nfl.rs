//! NFL规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AmericanFootballNflRules, name: "NFL规则", desc: "美国职业橄榄球联盟", origin: "美国", tags: ["体育", "球类"] }
impl AmericanFootballNflRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4次进攻10码"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["达阵6分"]
    }
}
impl Rule for AmericanFootballNflRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("american_football_nfl")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL规则",
            &[("进攻", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AmericanFootballNflRules::new();
        assert!(!r.explain().is_empty());
    }
}
