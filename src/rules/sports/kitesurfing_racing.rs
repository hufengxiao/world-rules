//! 风筝冲浪竞速
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KitesurfingRacingRules, name: "风筝冲浪竞速", desc: "风筝冲浪竞速规则", origin: "国际", tags: ["体育", "水上"] }
impl KitesurfingRacingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["障碍赛长距离"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["风力限制"]
    }
}
impl Rule for KitesurfingRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kitesurfing_racing")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "风筝冲浪竞速",
            &[("竞速", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KitesurfingRacingRules::new();
        assert!(!r.explain().is_empty());
    }
}
