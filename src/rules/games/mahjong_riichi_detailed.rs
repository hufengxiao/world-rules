//! 立直麻将详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MahjongRiichiDetailedRules, name: "立直麻将详细规则", desc: "日本立直麻将详细规则", origin: "日本", tags: ["游戏", "麻将"] }
impl MahjongRiichiDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["宣告立直", "一发", "里宝牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["役满", "振听规则", "流局"]
    }
}
impl Rule for MahjongRiichiDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_riichi_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "立直麻将详细规则",
            &[("立直", &self.section_0()), ("役", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongRiichiDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
