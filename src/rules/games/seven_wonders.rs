//! 七大奇迹规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SevenWondersRules, name: "七大奇迹规则", desc: "七大奇迹桌游规则", origin: "法国", tags: ["游戏", "桌游"] }
impl SevenWondersRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["三个时代", "卡牌轮转"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["军事", "科学", "商业"]
    }
}
impl Rule for SevenWondersRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("7_wonders")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "七大奇迹规则",
            &[("时代", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SevenWondersRules::new();
        assert!(!r.explain().is_empty());
    }
}
