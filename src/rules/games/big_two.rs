//! 大老二规则 (Big Two / 锄大D)
//!
//! 一种流行的亚洲扑克游戏，以2为最大牌。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BigTwoRules,
    name: "大老二规则",
    desc: "大老二(Big Two/锄大D)扑克游戏规则",
    origin: "中国",
    tags: ["游戏", "卡牌", "扑克"],
}

impl BigTwoRules {
    /// 游戏设置规则
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "4人游戏，每人发13张牌",
            "使用标准52张牌（无大小王）",
            "牌面大小：2 > A > K > Q > J > 10 > ... > 3",
            "花色大小：黑桃 > 红心 > 梅花 > 方块",
            "持有梅花3的玩家先出牌",
        ]
    }

    /// 牌型规则
    pub fn section_hands(&self) -> Vec<&'static str> {
        vec![
            "单张：任意单张牌",
            "对子：两张相同点数的牌",
            "三条：三张相同点数的牌",
            "顺子：五张连续点数（2不可用在顺子中）",
            "同花：五张同花色牌（不连续）",
            "葫芦：三条+对子",
            "铁支：四张相同点数+任意单张",
            "同花顺：五张同花色连续牌",
        ]
    }

    /// 出牌规则
    pub fn section_play(&self) -> Vec<&'static str> {
        vec![
            "必须跟牌型出牌（对子跟对子，顺子跟顺子）",
            "新出的牌必须大于上家的牌",
            "同一牌型大小比较：先比最大牌，再比花色",
            "无牌可出时可选择过牌",
            "当所有其他玩家都过牌时，出牌权归最后出牌者",
            "可随时出铁支或同花顺（炸弹）压制任意牌型",
        ]
    }

    /// 计分规则
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "率先出完所有牌者获胜",
            "剩1-2张牌：无惩罚",
            "剩3-5张牌：所剩张数×基础分",
            "剩6-9张牌：所剩张数×基础分×2",
            "剩10-12张牌：所剩张数×基础分×3",
            "剩13张（一张未出）：所剩张数×基础分×4",
            "基础分通常为1分",
        ]
    }
}

impl Rule for BigTwoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("big_two")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "大老二规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("牌型规则", &self.section_hands()),
                ("出牌规则", &self.section_play()),
                ("计分规则", &self.section_scoring()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_two_rules() {
        let rules = BigTwoRules::new();
        assert_eq!(rules.metadata().name, "大老二规则");
        assert!(rules.explain().contains("梅花3"));
        assert!(rules.explain().contains("铁支"));
    }

    #[test]
    fn test_big_two_category() {
        let rules = BigTwoRules::new();
        assert_eq!(rules.category(), RuleCategory::games("big_two"));
    }
}