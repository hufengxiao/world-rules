//! 贵阳麻将规则
//!
//! 贵阳麻将是贵州地区的代表玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 贵阳麻将规则
pub struct GuiyangMahjongRules {
    metadata: RuleMetadata,
}

impl GuiyangMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "贵阳麻将规则",
                "贵阳地区流行麻将规则"
            )
            .with_origin("贵阳")
            .with_tags(vec!["游戏".into(), "麻将".into(), "贵阳".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用108张牌(只有万条筒)",
            "每人起手13张",
            "必须缺一门",
            "只能自摸",
            "讲究杠牌",
        ]
    }

    /// 缺门规则
    pub fn quemen_rules(&self) -> Vec<&'static str> {
        vec![
            "开局选择缺哪一门",
            "缺万、缺条、缺筒",
            "必须有缺门才能胡",
            "与四川麻将类似",
            "增加策略性",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("对对胡", 2),
            ("清一色", 4),
            // 贵阳特色
            ("杠上花", 2),
            ("杠上炮", 2),
            ("清对", 4),
            ("将对", 4),
            // 高级番型
            ("七对", 2),
            ("清七对", 4),
            ("龙七对", 8),
            ("天胡", 10),
            ("地胡", 8),
        ]
    }

    /// 杠牌规则
    pub fn kong_rules(&self) -> Vec<&'static str> {
        vec![
            "明杠: 暴露杠牌",
            "暗杠: 隐藏杠牌",
            "杠后自摸: 杠上花",
            "杠后被点: 杠上炮",
            "杠牌是贵阳特色",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "不能点炮",
            "杠牌另计",
            "清一色番最高",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "只有万条筒",
            "必须缺一门",
            "只能自摸",
            "杠牌重要",
            "荒牌流局",
        ]
    }
}

impl Default for GuiyangMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GuiyangMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_guiyang")
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
            "【贵阳麻将规则】\n\n\
            基本设置:\n{}\n\n\
            缺门规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            杠牌规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.quemen_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.kong_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guiyang_mahjong_rules() {
        let rules = GuiyangMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}