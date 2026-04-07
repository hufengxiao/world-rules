//! 东北麻将规则
//!
//! 东北麻将特点是讲究"立棍"、"会牌"等独特玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 东北麻将规则
pub struct DongbeiMahjongRules {
    metadata: RuleMetadata,
}

impl DongbeiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "东北麻将规则",
                "东北地区流行麻将规则"
            )
            .with_origin("东北")
            .with_tags(vec!["游戏".into(), "麻将".into(), "东北".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究立棍、会牌",
        ]
    }

    /// 立棍规则
    pub fn ligen_rules(&self) -> Vec<&'static str> {
        vec![
            "立棍: 公开宣布听牌",
            "立棍后必须自摸",
            "立棍后不能换牌",
            "立棍胡牌番数翻倍",
            "立棍被胡要双倍赔付",
            "立棍增加刺激度",
        ]
    }

    /// 会牌规则
    pub fn hui_card_rules(&self) -> Vec<&'static str> {
        vec![
            "会牌: 翻开一张牌作为百搭",
            "会牌可代替任何牌",
            "会牌是翻牌数字的下一张",
            "如翻开5万，会牌是6万",
            "会牌增加胡牌概率",
            "会牌不能打出",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("对对胡", 2),
            ("混一色", 2),
            ("清一色", 4),
            ("七对子", 2),
            // 东北特色
            ("立棍胡", 4),
            ("清一色立棍", 8),
            ("大对子立棍", 6),
            // 高级番型
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
            ("抢杠", 2),
        ]
    }

    /// 闭门规则
    pub fn bimen_rules(&self) -> Vec<&'static str> {
        vec![
            "闭门: 不吃不碰不杠",
            "闭门胡牌番数加一",
            "闭门可以立棍",
            "吃碰后不能闭门",
            "闭门是东北麻将特色",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "立棍翻倍",
            "闭门加番",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "必须自摸才能胡",
            "点炮不能胡",
            "一人胡牌游戏结束",
            "荒牌流局",
            "会牌增加可玩性",
        ]
    }

    /// 连庄规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局庄家听牌连庄",
            "闲家胡牌下家做庄",
            "连庄增加番数",
        ]
    }
}

impl Default for DongbeiMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DongbeiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_dongbei")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let fan_list: String = self.fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【东北麻将规则】\n\n\
            基本设置:\n{}\n\n\
            立棍规则:\n{}\n\n\
            会牌规则:\n{}\n\n\
            番型规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.ligen_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.hui_card_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dongbei_mahjong_rules() {
        let rules = DongbeiMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.ligen_rules().is_empty());
    }
}