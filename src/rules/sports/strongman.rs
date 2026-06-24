//! 大力士赛规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: StrongmanRules, name: "大力士赛规则", desc: "世界大力士赛规则", origin: "国际", tags: ["体育", "力量"] }
impl StrongmanRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["硬拉推车"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["多项积分"]
    }
}
impl Rule for StrongmanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("strongman")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "大力士赛规则",
            &[("项目", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = StrongmanRules::new();
        assert!(!r.explain().is_empty());
    }
}
