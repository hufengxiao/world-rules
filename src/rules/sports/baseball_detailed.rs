//! 棒球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BaseballDetailedRules, name: "棒球详细规则", desc: "棒球详细比赛规则", origin: "MLB", tags: ["体育", "球类"] }
impl BaseballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["9局制", "延长赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["安打", "全垒打", "盗垒"]
    }
}
impl Rule for BaseballDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("baseball_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "棒球详细规则",
            &[("比赛", &self.section_0()), ("进攻", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BaseballDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
