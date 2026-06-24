//! 克里比奇规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CribbageRules, name: "克里比奇规则", desc: "克里比奇卡牌游戏", origin: "英国", tags: ["游戏", "卡牌"] }
impl CribbageRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["2人", "钉板计分", "凑15"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["15凑2分", "对子2分", "顺子"]
    }
}
impl Rule for CribbageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("cribbage")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "克里比奇规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CribbageRules::new();
        assert!(!r.explain().is_empty());
    }
}
