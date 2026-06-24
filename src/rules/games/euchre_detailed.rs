//! 尤克牌详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EuchreDetailedRules, name: "尤克牌详细规则", desc: "尤克牌详细规则", origin: "美国", tags: ["游戏", "卡牌"] }
impl EuchreDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["6人尤克", "双牌组"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["叫牌判断", "出牌顺序"]
    }
}
impl Rule for EuchreDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("euchre_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "尤克牌详细规则",
            &[("变体", &self.section_0()), ("策略", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EuchreDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
