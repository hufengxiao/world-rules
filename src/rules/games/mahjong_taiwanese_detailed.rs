//! 台湾麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongTaiwaneseDetailedRules, name: "台湾麻将详细规则", desc: "台湾十六张麻将", origin: "台湾", tags: ["游戏", "麻将"] }
impl MahjongTaiwaneseDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["16张手牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["台数计分"]
    }
}
impl Rule for MahjongTaiwaneseDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_taiwanese_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "台湾麻将详细规则",
            &[("16张", &self.section_0()), ("台数", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongTaiwaneseDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
