//! 客家麻将规则
//!
//! 客家麻将是客家地区的特色玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 客家麻将规则
pub struct KejiaMahjongRules {
    metadata: RuleMetadata,
}

impl KejiaMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "客家麻将规则",
                "客家地区流行麻将规则"
            )
            .with_origin("客家")
            .with_tags(vec!["游戏".into(), "麻将".into(), "客家".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究六八规则",
        ]
    }

    /// 六八规则
    pub fn liuba_rules(&self) -> Vec<&'static str> {
        vec![
            "六八: 特殊数字组合",
            "六八牌型有加成",
            "如68万、68条等",
            "客家特色规则",
            "增加趣味性",
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
            // 客家特色
            ("六八胡", 3),
            ("清六八", 5),
            ("混六八", 3),
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

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "六八加番",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "有六八规则",
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }
}

impl Default for KejiaMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KejiaMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_kejia")
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
            "【客家麻将规则】\n\n\
            基本设置:\n{}\n\n\
            六八规则:\n{}\n\n\
            番型规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.liuba_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kejia_mahjong_rules() {
        let rules = KejiaMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}