//! 西安麻将规则
//!
//! 西安麻将是陕西地区的代表玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 西安麻将规则
pub struct XianMahjongRules {
    metadata: RuleMetadata,
}

impl XianMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "西安麻将规则",
                "西安地区流行麻将规则"
            )
            .with_origin("西安")
            .with_tags(vec!["游戏".into(), "麻将".into(), "西安".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究定张规则",
        ]
    }

    /// 定张规则
    pub fn dingzhang_rules(&self) -> Vec<&'static str> {
        vec![
            "定张: 开局确定将牌",
            "将牌决定胡牌条件",
            "定张增加策略性",
            "不同将牌番数不同",
            "西安特色规则",
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
            // 西安特色
            ("定张胡", 2),
            ("清对", 6),
            ("混对", 4),
            // 高级番型
            ("十三幺", 10),
            ("大三元", 8),
            ("大四喜", 8),
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
            "定张加番",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "有定张规则",
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }
}

impl Default for XianMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for XianMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_xian")
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
            "【西安麻将规则】\n\n\
            基本设置:\n{}\n\n\
            定张规则:\n{}\n\n\
            番型规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.dingzhang_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xian_mahjong_rules() {
        let rules = XianMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}