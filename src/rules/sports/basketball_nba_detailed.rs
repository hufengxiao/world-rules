//! NBA详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballNbaDetailedRules, name: "NBA详细规则", desc: "NBA详细比赛规则", origin: "美国", tags: ["体育", "球类"] }
impl BasketballNbaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["选秀规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工资帽规则"]
    }
}
impl Rule for BasketballNbaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_nba_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA详细规则",
            &[("选秀", &self.section_0()), ("工资帽", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballNbaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
