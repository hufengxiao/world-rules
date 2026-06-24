//! 尤克牌规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EuchreRules, name: "尤克牌规则", desc: "尤克牌游戏规则", origin: "美国", tags: ["游戏", "卡牌"] }
impl EuchreRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4人2队", "王牌规则", "叫牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["每墩1分", "叫到并完成加分"]
    }
}
impl Rule for EuchreRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("euchre")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "尤克牌规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EuchreRules::new();
        assert!(!r.explain().is_empty());
    }
}
