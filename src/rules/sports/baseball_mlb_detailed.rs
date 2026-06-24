//! MLB详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BaseballMlbDetailedRules, name: "MLB详细规则", desc: "美国职业棒球大联盟", origin: "美国", tags: ["体育", "球类"] }
impl BaseballMlbDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["162场常规赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["外卡系列赛"]
    }
}
impl Rule for BaseballMlbDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("baseball_mlb_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB详细规则",
            &[("赛季", &self.section_0()), ("季后赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BaseballMlbDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
