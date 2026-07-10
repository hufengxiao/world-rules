//! 三张扑克规则 (Three Card Poker)
//!
//! 简单的赌场扑克游戏，玩家与庄家比较三张牌。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ThreeCardPokerRules,
    name: "三张扑克规则",
    desc: "三张扑克(Three Card Poker)规则，赌场游戏",
    origin: "美国",
    tags: ["游戏", "卡牌", "扑克", "赌场"],
}

impl ThreeCardPokerRules {
    /// 游戏设置规则
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "玩家先下底注(Ante)和/或对子注(Pair Plus)",
            "使用标准52张牌",
            "玩家和庄家各发3张牌",
            "玩家查看手牌后决定：弃牌或继续(Play)",
            "继续需下与底注等额的Play注",
        ]
    }

    /// 牌型大小
    pub fn section_hands(&self) -> Vec<&'static str> {
        vec![
            "三张牌型（从大到小）：",
            "  1. 同花顺：同花色连续三张（最高A-2-3）",
            "  2. 三条：三张相同点数",
            "  3. 顺子：连续三张",
            "  4. 同花：同花色三张",
            "  5. 对子：两张相同点数",
            "  6. 高牌：散牌",
            "比较规则：先比牌型，再比点数大小",
        ]
    }

    /// 对子注赔率
    pub fn section_pair_plus(&self) -> Vec<&'static str> {
        vec![
            "对子注独立于主游戏",
            "对子：赔率1:1",
            "同花：赔率4:1",
            "顺子：赔率6:1",
            "三条：赔率30:1",
            "同花顺：赔率40:1（部分赌场为35:1）",
            "无需击败庄家，只要有对子或更好牌型即可赢",
        ]
    }

    /// 主游戏规则
    pub fn section_main_game(&self) -> Vec<&'static str> {
        vec![
            "庄家必须有Q或更高才能开牌（资格牌）",
            "庄家无资格：底注1:1赔付，Play注退还",
            "庄家有资格：比较玩家与庄家牌型",
            "  - 玩家赢：底注1:1，Play注1:1",
            "  - 庄家赢：底注和Play注都输",
            "  - 平局：底注和Play注退还",
            "底注奖励(Ante Bonus)：无论输赢都给",
            "  - 同花顺：底注的5倍",
            "  - 三条：底注的4倍",
            "  - 顺子：底注的1倍",
        ]
    }

    /// 策略建议
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "基本策略：Q-6-4或更好时继续",
            "Q-6-4 = 女王高牌，次牌6，第三牌4",
            "低于Q-6-4时弃牌",
            "对子注独立考虑，期望值通常为负",
            "庄家无资格率约30%",
            "合理管理资金，设置止损线",
        ]
    }
}

impl Rule for ThreeCardPokerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("three_card_poker")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "三张扑克规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("牌型大小", &self.section_hands()),
                ("对子注赔率", &self.section_pair_plus()),
                ("主游戏规则", &self.section_main_game()),
                ("策略建议", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_card_poker_rules() {
        let rules = ThreeCardPokerRules::new();
        assert_eq!(rules.metadata().name, "三张扑克规则");
        assert!(rules.explain().contains("同花顺"));
        assert!(rules.explain().contains("Q-6-4"));
    }

    #[test]
    fn test_three_card_poker_category() {
        let rules = ThreeCardPokerRules::new();
        assert_eq!(rules.category(), RuleCategory::games("three_card_poker"));
    }
}