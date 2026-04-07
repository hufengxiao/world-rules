//! 福州麻将规则
//!
//! 福州麻将是福建地区的代表玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 福州麻将规则
pub struct FuzhouMahjongRules {
    metadata: RuleMetadata,
}

impl FuzhouMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "福州麻将规则",
                "福州地区流行麻将规则"
            )
            .with_origin("福州")
            .with_tags(vec!["游戏".into(), "麻将".into(), "福州".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "有花牌规则",
            "计分复杂",
        ]
    }

    /// 花牌规则
    pub fn flower_rules(&self) -> Vec<&'static str> {
        vec![
            "花牌: 春夏秋冬、梅兰竹菊",
            "花牌计番",
            "起手花牌亮出补牌",
            "花牌越多番数越高",
            "福州麻将特色",
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
            // 福州特色
            ("花胡", 2),
            ("三花", 3),
            ("四花", 4),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("大三元", 8),
            ("大四喜", 8),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 金牌规则
    pub fn jin_card_rules(&self) -> Vec<&'static str> {
        vec![
            "金牌: 翻开一张牌作为百搭",
            "金牌是翻牌的下一张",
            "金牌可代替任何牌",
            "金牌不能打出",
            "增加胡牌概率",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "花牌加番",
            "金牌减少番数",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "有花牌",
            "有金牌(百搭)",
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
        ]
    }
}

impl Default for FuzhouMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FuzhouMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_fuzhou")
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
            "【福州麻将规则】\n\n\
            基本设置:\n{}\n\n\
            花牌规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.flower_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzhou_mahjong_rules() {
        let rules = FuzhouMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}