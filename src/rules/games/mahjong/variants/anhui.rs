//! 安徽麻将规则
//!
//! 安徽麻将是安徽地区的代表玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 安徽麻将规则
pub struct AnhuiMahjongRules {
    metadata: RuleMetadata,
}

impl AnhuiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "安徽麻将规则",
                "安徽地区流行麻将规则"
            )
            .with_origin("安徽")
            .with_tags(vec!["游戏".into(), "麻将".into(), "安徽".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究飘胡规则",
        ]
    }

    /// 飘胡规则
    pub fn piaohu_rules(&self) -> Vec<&'static str> {
        vec![
            "飘胡: 门清自摸",
            "飘胡番数较高",
            "必须不吃不碰不杠",
            "必须自摸",
            "安徽特色规则",
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
            // 安徽特色
            ("飘胡", 4),
            ("清飘", 6),
            ("混飘", 4),
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
            "飘胡翻倍",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "有飘胡规则",
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }
}

impl Default for AnhuiMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AnhuiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_anhui")
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
            "【安徽麻将规则】\n\n\
            基本设置:\n{}\n\n\
            飘胡规则:\n{}\n\n\
            番型规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.piaohu_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anhui_mahjong_rules() {
        let rules = AnhuiMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}