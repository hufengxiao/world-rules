//! 黑桃王规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SpadesRules, name: "黑桃王规则", desc: "黑桃王卡牌游戏", origin: "美国", tags: ["游戏", "卡牌"] }
impl SpadesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4人2队", "叫牌", "黑桃永远是王牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["叫到10得100分", "超额10分/墩"]
    }
}
impl Rule for SpadesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("spades")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "黑桃王规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SpadesRules::new();
        assert!(!r.explain().is_empty());
    }
}
