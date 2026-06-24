//! 皮诺克尔规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PinochleRules, name: "皮诺克尔规则", desc: "皮诺克尔卡牌游戏", origin: "德国", tags: ["游戏", "卡牌"] }
impl PinochleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["48张牌", "叫牌", "王牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["组合分", "墩分"]
    }
}
impl Rule for PinochleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("pinochle")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "皮诺克尔规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PinochleRules::new();
        assert!(!r.explain().is_empty());
    }
}
