//! 台湾麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongTaiwaneseDetailedRules, name: "台湾麻将详细规则", desc: "台湾十六张麻将", origin: "台湾", tags: ["游戏", "麻将"] }
impl MahjongTaiwaneseDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "台湾麻将使用16张手牌",
            "使用144张牌含花牌",
            "每人发16张牌比一般麻将多3张",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "以台数计算番值",
            "花牌可加台",
            "门清自摸加台",
            "各种特殊牌型有不同台数",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "连庄规则:庄家胡牌可连庄",
            "花牌补牌规则",
            "台数门槛通常为8台或16台",
        ]
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
            &[
                ("基本规则", &self.section_0()),
                ("计分规则", &self.section_1()),
                ("特殊规则", &self.section_2()),
            ],
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
