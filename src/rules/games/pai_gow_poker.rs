//! 牌九扑克规则 (Pai Gow Poker)
//!
//! 中国牌九与美国扑克的结合，在赌场中非常流行。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PaiGowPokerRules,
    name: "牌九扑克规则",
    desc: "牌九扑克(Pai Gow Poker)规则，结合中国牌九与西方扑克",
    origin: "美国",
    tags: ["游戏", "卡牌", "扑克", "赌场"],
}

impl PaiGowPokerRules {
    /// 游戏设置规则
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "1-7人玩家对战庄家",
            "使用53张牌（标准52张+1张鬼牌）",
            "每人发7张牌",
            "玩家将7张牌分成两手：5张（高牌）和2张（低牌）",
            "庄家按庄家规则（House Way）分牌",
        ]
    }

    /// 牌型大小
    pub fn section_hands(&self) -> Vec<&'static str> {
        vec![
            "五张牌型（从大到小）：",
            "  1. 五鬼：四张鬼牌+鬼牌（唯一最大）",
            "  2. 皇家同花顺：同花A-K-Q-J-10",
            "  3. 同花顺：同花色连续五张",
            "  4. 四条：四张相同点数",
            "  5. 葫芦：三条+对子",
            "  6. 同花：同花色五张",
            "  7. 顺子：连续五张",
            "  8. 三条：三张相同点数",
            "  9. 两对：两个对子",
            "  10. 一对：一个对子",
            "  11. 高牌：散牌",
            "两张牌型：对子 > 高牌",
        ]
    }

    /// 分牌规则
    pub fn section_arrangement(&self) -> Vec<&'static str> {
        vec![
            "五张牌组必须大于两张牌组",
            "若五张牌组小于两张牌组，则算自动输牌",
            "鬼牌可当百搭牌，当作任意花色或点数",
            "鬼牌也可当A使用，完成A-2-3-4-5顺子",
            "玩家可选择自行分牌或请庄家代分",
        ]
    }

    /// 比牌规则
    pub fn section_comparison(&self) -> Vec<&'static str> {
        vec![
            "玩家与庄家分别比较高低牌组",
            "双赢：玩家高低牌组都赢，玩家赢1倍注金",
            "双输：玩家高低牌组都输，玩家输掉注金",
            "平局：一赢一输或两平，庄家赢",
            "若高低牌组都平，庄家赢",
        ]
    }

    /// 庄家规则
    pub fn section_house_way(&self) -> Vec<&'static str> {
        vec![
            "庄家必须按固定规则分牌：",
            "无对子：按最高牌分牌",
            "一对：保留五张，另两张为低牌组",
            "两对：按特定规则分牌（复杂）",
            "三对：最高对放两牌组，另两对放五牌组",
            "三张相同：两对规则处理",
            "两副三条：一副放五牌组，一副分开",
            "四张相同：按两对规则处理",
        ]
    }
}

impl Rule for PaiGowPokerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("pai_gow_poker")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "牌九扑克规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("牌型大小", &self.section_hands()),
                ("分牌规则", &self.section_arrangement()),
                ("比牌规则", &self.section_comparison()),
                ("庄家规则", &self.section_house_way()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pai_gow_poker_rules() {
        let rules = PaiGowPokerRules::new();
        assert_eq!(rules.metadata().name, "牌九扑克规则");
        assert!(rules.explain().contains("五鬼"));
        assert!(rules.explain().contains("庄家规则"));
    }

    #[test]
    fn test_pai_gow_poker_category() {
        let rules = PaiGowPokerRules::new();
        assert_eq!(rules.category(), RuleCategory::games("pai_gow_poker"));
    }
}
