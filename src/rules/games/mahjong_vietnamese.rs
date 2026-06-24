//! 越南麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongVietnameseRules, name: "越南麻将规则", desc: "越南麻将规则", origin: "越南", tags: ["游戏", "麻将"] }
impl MahjongVietnameseRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["16张麻将"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特殊牌型"]
    }
}
impl Rule for MahjongVietnameseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_vietnamese")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "越南麻将规则",
            &[("基本", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongVietnameseRules::new();
        assert!(!r.explain().is_empty());
    }
}
