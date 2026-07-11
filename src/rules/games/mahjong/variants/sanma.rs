//! 三人麻将规则
//!
//! 日本三人麻将变体，适合3人对局

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 三人麻将规则
pub struct SanmaMahjongRules {
    metadata: RuleMetadata,
}

impl Default for SanmaMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl SanmaMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("三人麻将规则", "日本三人麻将变体")
                .with_origin("日本")
                .with_tags(vec!["游戏".into(), "麻将".into(), "三人".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用108张牌(去掉2-8万)",
            "三人游戏，每人起手13张",
            "保留所有字牌",
            "保留筒和条",
            "加快游戏节奏",
        ]
    }

    /// 牌型调整
    pub fn tile_adjustments(&self) -> Vec<&'static str> {
        vec![
            "去除2-8万(保留1万和9万)",
            "保留所有风牌和三元牌",
            "保留完整筒和条",
            "减少牌型组合",
            "提高特定役种概率",
        ]
    }

    /// 特殊役种(格式化)
    pub fn special_yaku_formatted(&self) -> Vec<&'static str> {
        vec![
            "立直: 1番",
            "一发: 1番",
            "门前清自摸和: 1番",
            "平和: 1番",
            "断幺九: 1番",
            "役牌: 1番",
            "三色同刻: 2番",
            "一气通贯: 2番",
            "对对和: 2番",
            "三暗刻: 2番",
            "混全带幺九: 2番",
            "七对子: 2番",
            "北风处理: 特殊",
        ]
    }

    /// 特殊役种
    pub fn special_yaku(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基础役(同标准立直)
            ("立直", 1),
            ("断幺九", 1),
            ("平和", 1),
            // 三人麻将特色
            ("三色同顺(筒条)", 2),
            ("一气通贯", 2),
            ("混全带幺九", 2),
            ("七对子", 2),
            ("对对和", 2),
            // 三人麻将高概率役
            ("混一色", 2),
            ("清一色", 5),
            // 役满
            ("大三元", 13),
            ("字一色", 13),
            ("清老头", 13),
        ]
    }

    /// 北风处理
    pub fn north_wind_rules(&self) -> Vec<&'static str> {
        vec![
            "北风可作为役牌",
            "三家各有自风",
            "北风番数调整",
            "可设置北风特殊规则",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "基本计分同立直",
            "减少一人影响支付",
            "庄家制度保留",
            "供托点数调整",
            "三家分摊",
        ]
    }

    /// 游戏节奏
    pub fn game_pace(&self) -> Vec<&'static str> {
        vec![
            "游戏节奏更快",
            "流局概率降低",
            "胡牌率提高",
            "适合快速对局",
            "训练防守技巧",
        ]
    }

    /// 策略要点
    pub fn strategy_points(&self) -> Vec<&'static str> {
        vec![
            "牌池小，需精确计算",
            "一色更容易成型",
            "防守更重要",
            "注意剩余牌数",
            "三家博弈更直接",
        ]
    }
}

impl Rule for SanmaMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_sanma")
    }

    fn validate(
        &self,
        _ctx: &crate::rules::core::ValidateContext,
    ) -> crate::rules::core::RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "三人麻将规则",
            &[
                ("基本设置", &self.basic_settings()),
                ("牌型调整", &self.tile_adjustments()),
                ("特殊役种", &self.special_yaku_formatted()),
                ("北风处理", &self.north_wind_rules()),
                ("计分规则", &self.scoring_rules()),
                ("游戏节奏", &self.game_pace()),
                ("策略要点", &self.strategy_points()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = SanmaMahjongRules::new();
        assert_eq!(rules.metadata().name, "三人麻将规则");
        assert!(!rules.explain().is_empty());
    }
}
