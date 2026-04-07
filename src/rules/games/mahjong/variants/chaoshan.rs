//! 潮汕麻将规则
//!
//! 潮汕麻将是广东潮汕地区的特色玩法，讲究"番"的计算

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 潮汕麻将规则
pub struct ChaoshanMahjongRules {
    metadata: RuleMetadata,
}

impl ChaoshanMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "潮汕麻将规则",
                "潮汕地区流行麻将规则"
            )
            .with_origin("潮汕")
            .with_tags(vec!["游戏".into(), "麻将".into(), "潮汕".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "番数计算复杂",
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
            // 潮汕特色
            ("三番", 3),
            ("六番", 6),
            ("九番", 9),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("大三元", 8),
            ("小三元", 6),
            ("大四喜", 8),
            ("小四喜", 6),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 三六九番
    pub fn sanliujiu_rules(&self) -> Vec<&'static str> {
        vec![
            "三番: 基础番数",
            "六番: 中等番数",
            "九番: 高等番数",
            "番数决定赔付",
            "可叠加计算",
        ]
    }

    /// 特殊胡牌
    pub fn special_wins(&self) -> Vec<&'static str> {
        vec![
            "天胡: 庄家起手14张胡",
            "地胡: 闲家第一圈自摸",
            "人胡: 第一圈点炮胡",
            "杠开: 杠后自摸",
            "抢杠: 抢他人补杠",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "番数叠加计算",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
            "连庄规则",
        ]
    }

    /// 连庄规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局庄家听牌连庄",
            "闲家胡牌下家做庄",
            "连庄增加番数",
            "最多连庄四圈",
        ]
    }
}

impl Default for ChaoshanMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChaoshanMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_chaoshan")
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
            "【潮汕麻将规则】\n\n\
            基本设置:\n{}\n\n\
            番型规则:\n{}\n\n\
            三六九番:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.sanliujiu_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaoshan_mahjong_rules() {
        let rules = ChaoshanMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.fan_types().is_empty());
    }
}