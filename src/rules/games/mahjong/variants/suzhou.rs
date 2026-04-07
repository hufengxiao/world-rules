//! 苏州麻将规则
//!
//! 苏州麻将是江苏地区的特色玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 苏州麻将规则
pub struct SuzhouMahjongRules {
    metadata: RuleMetadata,
}

impl SuzhouMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "苏州麻将规则",
                "苏州地区流行麻将规则"
            )
            .with_origin("苏州")
            .with_tags(vec!["游戏".into(), "麻将".into(), "苏州".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究门清规则",
        ]
    }

    /// 门清规则
    pub fn menqing_rules(&self) -> Vec<&'static str> {
        vec![
            "门清: 不吃不碰不杠",
            "门清胡牌番数加成",
            "门清是苏州特色",
            "增加策略性",
            "讲究防守",
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
            // 苏州特色
            ("门清", 1),
            ("门清自摸", 2),
            ("清门清", 5),
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
            "门清加番",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "有门清规则",
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }
}

impl Default for SuzhouMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SuzhouMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_suzhou")
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
            "【苏州麻将规则】\n\n\
            基本设置:\n{}\n\n\
            门清规则:\n{}\n\n\
            番型规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.menqing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suzhou_mahjong_rules() {
        let rules = SuzhouMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}