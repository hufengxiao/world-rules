//! 宁夏麻将规则
//!
//! 宁夏麻将特点是"碰碰胡"盛行，强调对子组合

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 宁夏麻将规则
pub struct NingxiaMahjongRules {
    metadata: RuleMetadata,
}

impl NingxiaMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("宁夏麻将规则", "宁夏自治区地区流行麻将规则")
                .with_origin("宁夏")
                .with_tags(vec!["游戏".into(), "麻将".into(), "宁夏".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "碰碰胡盛行",
        ]
    }

    /// 碰碰胡规则
    pub fn pengpeng_hu_rules(&self) -> Vec<&'static str> {
        vec![
            "碰碰胡: 4刻子+1对子",
            "番数较高",
            "强调碰牌策略",
            "碰后易成碰碰胡",
            "宁夏麻将特色",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("碰碰胡", 2),
            ("七对子", 2),
            // 花色番型
            ("混一色", 2),
            ("清一色", 4),
            ("碰碰清一色", 6),
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
            "碰碰胡庄家优势大",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "庄家翻倍",
            "碰碰胡加番",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "碰牌策略重要",
            "可以明杠暗杠",
            "杠后补牌",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "一家胡牌后游戏结束",
            "碰碰胡常见",
            "荒牌流局",
            "可以抢杠胡",
            "强调碰牌",
        ]
    }
}

impl Default for NingxiaMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NingxiaMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_ningxia")
    }

    fn explain(&self) -> String {
        let fan_list: String = self
            .fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【宁夏麻将规则】\n\n\
            基本设置:\n{}\n\n\
            碰碰胡规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pengpeng_hu_rules()
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
    fn test_ningxia_mahjong_rules() {
        let rules = NingxiaMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.pengpeng_hu_rules().is_empty());
        assert!(rules.fan_types().len() > 0);
    }
}
