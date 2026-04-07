//! 海南麻将规则
//!
//! 海南麻将是海南地区的特色玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 海南麻将规则
pub struct HainanMahjongRules {
    metadata: RuleMetadata,
}

impl HainanMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "海南麻将规则",
                "海南地区流行麻将规则"
            )
            .with_origin("海南")
            .with_tags(vec!["游戏".into(), "麻将".into(), "海南".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究杠牌规则",
        ]
    }

    /// 杠牌规则
    pub fn kong_rules(&self) -> Vec<&'static str> {
        vec![
            "明杠: 暴露杠牌",
            "暗杠: 隐藏杠牌",
            "杠牌有额外计分",
            "杠后可自摸胡",
            "杠牌是海南特色",
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
            // 海南特色
            ("杠上花", 2),
            ("杠上炮", 2),
            ("清杠", 4),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("大三元", 8),
            ("大四喜", 8),
            ("天胡", 10),
            ("地胡", 8),
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "杠牌另计",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "杠牌重要",
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }
}

impl Default for HainanMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HainanMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hainan")
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
            "【海南麻将规则】\n\n\
            基本设置:\n{}\n\n\
            杠牌规则:\n{}\n\n\
            番型规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.kong_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hainan_mahjong_rules() {
        let rules = HainanMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}