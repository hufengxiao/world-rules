//! 南昌麻将规则
//!
//! 南昌麻将是江西地区的代表玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 南昌麻将规则
pub struct NanchangMahjongRules {
    metadata: RuleMetadata,
}

impl NanchangMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "南昌麻将规则",
                "南昌地区流行麻将规则"
            )
            .with_origin("南昌")
            .with_tags(vec!["游戏".into(), "麻将".into(), "南昌".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "讲究冲规则",
            "计分独特",
        ]
    }

    /// 冲规则
    pub fn chong_rules(&self) -> Vec<&'static str> {
        vec![
            "冲: 点炮胡牌",
            "冲番: 点炮者多付",
            "自冲: 自摸胡牌",
            "冲是南昌特色",
            "增加刺激性",
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
            // 南昌特色
            ("冲胡", 1),
            ("自冲", 2),
            ("杠冲", 3),
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
            "点炮者多付",
            "冲番额外计算",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃碰杠",
            "可以点炮",
            "冲番重要",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }

    /// 吃碰规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰影响番型",
            "听牌后不能再操作",
        ]
    }
}

impl Default for NanchangMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NanchangMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_nanchang")
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
            "【南昌麻将规则】\n\n\
            基本设置:\n{}\n\n\
            冲规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.chong_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nanchang_mahjong_rules() {
        let rules = NanchangMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}