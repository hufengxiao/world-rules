//! 加勒比斯图德扑克规则 (Caribbean Stud Poker)
//!
//! 赌场扑克游戏，玩家与庄家比较五张牌。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CaribbeanStudRules,
    name: "加勒比斯图德扑克规则",
    desc: "加勒比斯图德扑克(Caribbean Stud Poker)规则，赌场游戏",
    origin: "加勒比海",
    tags: ["游戏", "卡牌", "扑克", "赌场"],
}

impl CaribbeanStudRules {
    /// 游戏设置规则
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "玩家先下底注(Ante)",
            "可选择下渐进奖池注(Progressive Jackpot)",
            "使用标准52张牌",
            "玩家和庄家各发5张牌",
            "庄家一张牌面朝上，其余面朝下",
            "玩家可查看自己的5张牌",
        ]
    }

    /// 牌型大小
    pub fn section_hands(&self) -> Vec<&'static str> {
        vec![
            "五张牌型（从大到小）：",
            "  1. 皇家同花顺：同花A-K-Q-J-10",
            "  2. 同花顺：同花色连续五张",
            "  3. 四条：四张相同点数",
            "  4. 葫芦：三条+对子",
            "  5. 同花：同花色五张",
            "  6. 顺子：连续五张",
            "  7. 三条：三张相同点数",
            "  8. 两对：两个对子",
            "  9. 一对：一个对子",
            "  10. 高牌：散牌",
        ]
    }

    /// 游戏流程
    pub fn section_gameplay(&self) -> Vec<&'static str> {
        vec![
            "查看手牌后决定：弃牌或继续(Raise)",
            "弃牌：底注输掉，游戏结束",
            "继续：下双倍底注的加注(Raise)",
            "庄家开牌，必须至少有A-K或更好才能开牌",
            "庄家无资格：底注1:1赔付，加注退还",
            "庄家有资格：比较玩家与庄家牌型",
            "  - 玩家赢：底注1:1，加注按牌型赔率",
            "  - 庄家赢：底注和加注都输",
            "  - 平局：底注和加注退还",
        ]
    }

    /// 加注赔率
    pub fn section_payout(&self) -> Vec<&'static str> {
        vec![
            "玩家胜利后加注赔付（仅当庄家有资格）：",
            "  - 高牌或一对：1:1",
            "  - 两对：2:1",
            "  - 三条：3:1",
            "  - 顺子：4:1",
            "  - 同花：5:1",
            "  - 葫芦：7:1",
            "  - 四条：20:1",
            "  - 同花顺：50:1",
            "  - 皇家同花顺：100:1",
        ]
    }

    /// 渐进奖池
    pub fn section_progressive(&self) -> Vec<&'static str> {
        vec![
            "渐进奖池注通常为$1或固定金额",
            "累计奖池无论庄家是否有资格都可获得：",
            "  - 皇家同花顺：100%奖池",
            "  - 同花顺：10%奖池",
            "  - 四条：$500或固定金额",
            "  - 葫芦：$100或固定金额",
            "  - 同花：$75或固定金额",
            "  - 顺子：$50或固定金额",
            "注：渐进奖池规则因赌场而异",
        ]
    }
}

impl Rule for CaribbeanStudRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("caribbean_stud")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "加勒比斯图德扑克规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("牌型大小", &self.section_hands()),
                ("游戏流程", &self.section_gameplay()),
                ("加注赔率", &self.section_payout()),
                ("渐进奖池", &self.section_progressive()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caribbean_stud_rules() {
        let rules = CaribbeanStudRules::new();
        assert_eq!(rules.metadata().name, "加勒比斯图德扑克规则");
        assert!(rules.explain().contains("皇家同花顺"));
        assert!(rules.explain().contains("渐进奖池"));
    }

    #[test]
    fn test_caribbean_stud_category() {
        let rules = CaribbeanStudRules::new();
        assert_eq!(rules.category(), RuleCategory::games("caribbean_stud"));
    }
}
