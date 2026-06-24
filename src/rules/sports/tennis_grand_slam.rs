//! 大满贯规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TennisGrandSlamRules, name: "大满贯规则", desc: "网球大满贯赛事规则", origin: "国际", tags: ["体育", "球类"] }
impl TennisGrandSlamRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["澳网法网温网美网"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["5盘3胜"]
    }
}
impl Rule for TennisGrandSlamRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tennis_grand_slam")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "大满贯规则",
            &[("赛事", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TennisGrandSlamRules::new();
        assert!(!r.explain().is_empty());
    }
}
