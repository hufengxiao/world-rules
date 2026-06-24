//! 虫虫蜂房规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HiveRules, name: "虫虫蜂房规则", desc: "虫虫蜂房桌游规则", origin: "国际", tags: ["游戏", "棋类"] }
impl HiveRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["蜂后蚂蚁甲虫蜘蛛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["围住对方蜂后"]
    }
}
impl Rule for HiveRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("hive")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "虫虫蜂房规则",
            &[("棋子", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HiveRules::new();
        assert!(!r.explain().is_empty());
    }
}
