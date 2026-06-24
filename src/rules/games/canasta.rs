//! 卡纳斯塔规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CanastaRules, name: "卡纳斯塔规则", desc: "卡纳斯塔卡牌游戏", origin: "乌拉圭", tags: ["游戏", "卡牌"] }
impl CanastaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["2-6人", "红黑百搭", "组牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["自然组", "混合组", "清卡"]
    }
}
impl Rule for CanastaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("canasta")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "卡纳斯塔规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CanastaRules::new();
        assert!(!r.explain().is_empty());
    }
}
