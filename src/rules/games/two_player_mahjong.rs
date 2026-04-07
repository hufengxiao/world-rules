//! 二人麻将规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 二人麻将规则
pub struct TwoPlayerMahjongRules {
    metadata: RuleMetadata,
}

impl TwoPlayerMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "二人麻将规则",
                "二人麻将游戏规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "麻将".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "两人对战",
            "使用简化的牌组",
            "去掉万子或条子",
            "每人13张牌",
        ]
    }

    /// 游戏特点
    pub fn game_features(&self) -> Vec<&'static str> {
        vec![
            "节奏更快",
            "减少牌种，增加胡牌概率",
            "适合手机游戏",
            "番型简化",
        ]
    }

    /// 胡牌条件
    pub fn winning_conditions(&self) -> Vec<&'static str> {
        vec![
            "标准型: 4组+1对",
            "七对子",
            "番数要求较低",
        ]
    }

    /// 与四人麻将区别
    pub fn differences(&self) -> Vec<&'static str> {
        vec![
            "不能吃牌",
            "只能碰和杠",
            "对局更快",
            "计算简化",
        ]
    }
}

impl Default for TwoPlayerMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TwoPlayerMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("two_player_mahjong")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【二人麻将规则】\n\n\
            基本设置:\n{}\n\n\
            游戏特点:\n{}\n\n\
            与四人麻将区别:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.game_features().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.differences().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_player_mahjong_rules() {
        let rules = TwoPlayerMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}