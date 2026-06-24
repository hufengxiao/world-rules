//! 日本麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongJapaneseDetailedRules, name: "日本麻将详细规则", desc: "日本立直麻将", origin: "日本", tags: ["游戏", "麻将"] }
impl MahjongJapaneseDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["立直条件", "一发", "里宝牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["大四喜", "绿一色"]
    }
}
impl Rule for MahjongJapaneseDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_japanese_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "日本麻将详细规则",
            &[("立直", &self.section_0()), ("役满", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongJapaneseDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
