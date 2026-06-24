//! 璀璨宝石规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SplendorRules, name: "璀璨宝石规则", desc: "璀璨宝石桌游规则", origin: "国际", tags: ["游戏", "桌游"] }
impl SplendorRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["收集宝石", "购买发展卡"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先到15分"]
    }
}
impl Rule for SplendorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("splendor")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "璀璨宝石规则",
            &[("基本", &self.section_0()), ("目标", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SplendorRules::new();
        assert!(!r.explain().is_empty());
    }
}
