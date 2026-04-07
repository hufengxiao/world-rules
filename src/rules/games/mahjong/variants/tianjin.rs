//! 天津麻将规则
//!
//! 天津麻将特点是"混儿"规则，计分方式独特

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 天津麻将规则
pub struct TianjinMahjongRules {
    metadata: RuleMetadata,
}

impl TianjinMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "天津麻将规则",
                "天津地区流行麻将规则"
            )
            .with_origin("天津")
            .with_tags(vec!["游戏".into(), "麻将".into(), "天津".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "有混儿(百搭牌)规则",
            "只能自摸胡牌",
        ]
    }

    /// 混儿规则
    pub fn hunr_rules(&self) -> Vec<&'static str> {
        vec![
            "混儿: 翻开一张牌作为百搭",
            "混儿是翻牌数字的下一张",
            "如翻开7万，混儿是8万",
            "混儿可代替任何牌",
            "混儿不能打出",
            "有混儿番数减少",
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
            // 混儿相关
            ("有混儿", 1),
            ("无混儿", 2),
            ("双混儿", 3),
            // 高级番型
            ("捉五", 3),
            ("龙", 6),
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 捉五规则
    pub fn zhuowu_rules(&self) -> Vec<&'static str> {
        vec![
            "捉五: 胡牌中有5万、5条或5筒",
            "捉五是天津特色",
            "捉五番数加成",
            "捉五龙更高番",
            "增加策略性",
        ]
    }

    /// 龙规则
    pub fn long_rules(&self) -> Vec<&'static str> {
        vec![
            "龙: 同花色1-9连牌",
            "龙是高番番型",
            "清龙更高番",
            "混龙次之",
            "龙增加观赏性",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "不能点炮",
            "无混儿翻倍",
            "龙番数最高",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "必须自摸",
            "混儿翻开确定",
            "一家胡牌结束",
            "荒牌流局",
            "可以报听",
        ]
    }
}

impl Default for TianjinMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TianjinMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_tianjin")
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
            "【天津麻将规则】\n\n\
            基本设置:\n{}\n\n\
            混儿规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.hunr_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tianjin_mahjong_rules() {
        let rules = TianjinMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.hunr_rules().is_empty());
    }
}