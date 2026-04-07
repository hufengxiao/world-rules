//! 郑州麻将规则
//!
//! 郑州麻将是河南地区的代表玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 郑州麻将规则
pub struct ZhengzhouMahjongRules {
    metadata: RuleMetadata,
}

impl ZhengzhouMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "郑州麻将规则",
                "郑州地区流行麻将规则"
            )
            .with_origin("郑州")
            .with_tags(vec!["游戏".into(), "麻将".into(), "郑州".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究混牌规则",
        ]
    }

    /// 混牌规则
    pub fn hunpai_rules(&self) -> Vec<&'static str> {
        vec![
            "混牌: 翻开一张牌作为百搭",
            "混牌可代替任何牌",
            "有混牌番数减少",
            "无混牌番数增加",
            "增加胡牌概率",
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
            // 郑州特色
            ("有混", 1),
            ("无混", 2),
            ("清混", 3),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "无混翻倍",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "有混牌规则",
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }
}

impl Default for ZhengzhouMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ZhengzhouMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_zhengzhou")
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
            "【郑州麻将规则】\n\n\
            基本设置:\n{}\n\n\
            混牌规则:\n{}\n\n\
            番型规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.hunpai_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zhengzhou_mahjong_rules() {
        let rules = ZhengzhouMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}