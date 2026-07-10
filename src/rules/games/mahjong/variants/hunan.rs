//! 湖南麻将规则
//!
//! 湖南麻将特点是"七小对"盛行，玩法灵活多变

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 湖南麻将规则
pub struct HunanMahjongRules {
    metadata: RuleMetadata,
}

impl HunanMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("湖南麻将规则", "湖南省地区流行麻将规则")
                .with_origin("湖南")
                .with_tags(vec!["游戏".into(), "麻将".into(), "湖南".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "七小对盛行",
        ]
    }

    /// 七小对规则
    pub fn seven_pairs_rules(&self) -> Vec<&'static str> {
        vec![
            "七小对: 7个对子胡牌",
            "七小对番数较高",
            "龙七对: 有一杠的七小对",
            "双龙七对: 两杠的七小对",
            "湖南麻将七小对常见",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("对对胡", 2),
            ("七小对", 2),
            // 花色番型
            ("混一色", 2),
            ("清一色", 4),
            // 七小对变体
            ("龙七对", 4),
            ("双龙七对", 8),
            // 高级番型
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
            ("抢杠", 2),
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局庄家听牌连庄",
            "闲家胡牌轮庄",
            "庄家番数翻倍",
            "点炮庄家付双倍",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "庄家翻倍",
            "七小对加番",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰后可继续胡",
            "杠后摸打",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "一家胡牌后游戏结束",
            "可以抢杠胡",
            "可以海底捞月",
            "荒牌流局",
            "听牌可报",
        ]
    }
}

impl Default for HunanMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HunanMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hunan")
    }

    fn explain(&self) -> String {
        let fan_list: String = self
            .fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【湖南麻将规则】\n\n\
            基本设置:\n{}\n\n\
            七小对规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.seven_pairs_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            fan_list,
            self.scoring_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hunan_mahjong_rules() {
        let rules = HunanMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.seven_pairs_rules().is_empty());
        assert!(rules.fan_types().len() > 0);
    }
}
