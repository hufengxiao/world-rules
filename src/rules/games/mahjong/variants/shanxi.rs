//! 山西麻将规则
//!
//! 山西麻将特点是"缺一门"玩法，讲究策略性

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 山西麻将规则
pub struct ShanxiMahjongRules {
    metadata: RuleMetadata,
}

impl ShanxiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("山西麻将规则", "山西省地区流行麻将规则")
                .with_origin("山西")
                .with_tags(vec!["游戏".into(), "麻将".into(), "山西".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究缺一门策略",
        ]
    }

    /// 缺一门规则
    pub fn quemen_rules(&self) -> Vec<&'static str> {
        vec![
            "缺一门: 胡牌时只有两种花色",
            "增加胡牌难度",
            "提高策略性",
            "缺门可自选或强制",
            "山西麻将特色",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("缺一门", 1),
            ("对对胡", 2),
            // 花色番型
            ("清一色", 4),
            ("缺门清一色", 6),
            ("七对子", 2),
            // 高级番型
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
            ("海底捞", 2),
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局庄家听牌连庄",
            "闲家胡牌轮庄",
            "庄家番数翻倍",
            "庄家优先",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "庄家翻倍",
            "缺一门加番",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "杠后补牌",
            "吃碰可继续胡",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "一家胡牌后游戏结束",
            "缺一门增加难度",
            "荒牌流局",
            "听牌可报",
            "讲究策略",
        ]
    }
}

impl Default for ShanxiMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShanxiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_shanxi")
    }

    fn explain(&self) -> String {
        let fan_list: String = self
            .fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【山西麻将规则】\n\n\
            基本设置:\n{}\n\n\
            缺一门规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.quemen_rules()
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
    fn test_shanxi_mahjong_rules() {
        let rules = ShanxiMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.quemen_rules().is_empty());
        assert!(rules.fan_types().len() > 0);
    }
}
