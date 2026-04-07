//! 杭州麻将规则
//!
//! 杭州麻将是浙江地区流行的玩法，讲究"财神"规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 杭州麻将规则
pub struct HangzhouMahjongRules {
    metadata: RuleMetadata,
}

impl HangzhouMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "杭州麻将规则",
                "杭州地区流行麻将规则"
            )
            .with_origin("杭州")
            .with_tags(vec!["游戏".into(), "麻将".into(), "杭州".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "有财神(百搭牌)规则",
            "财神翻开后游戏开始",
        ]
    }

    /// 财神规则
    pub fn caishen_rules(&self) -> Vec<&'static str> {
        vec![
            "财神: 翻开一张牌作为百搭",
            "财神是翻牌的下一张",
            "如翻开3条，财神是4条",
            "财神可代替任何牌",
            "财神不能打出",
            "财神可增加胡牌概率",
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
            // 财神相关
            ("财神胡", 1),
            ("无财神", 2),
            ("双财神", 3),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 财神计算
    pub fn caishen_calculation(&self) -> Vec<&'static str> {
        vec![
            "有财神: 正常计算番数",
            "无财神: 番数翻倍",
            "多个财神: 番数累加",
            "财神只能用于胡牌",
            "财神不能打出去",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "无财神翻倍",
            "财神增加策略性",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "财神翻开确定百搭",
            "可以点炮胡牌",
            "一家胡牌游戏结束",
            "荒牌流局",
            "财神不能打",
        ]
    }

    /// 吃碰规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "杠后摸打财神",
            "听牌后不能再操作",
        ]
    }
}

impl Default for HangzhouMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HangzhouMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hangzhou")
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
            "【杭州麻将规则】\n\n\
            基本设置:\n{}\n\n\
            财神规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.caishen_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hangzhou_mahjong_rules() {
        let rules = HangzhouMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.caishen_rules().is_empty());
    }
}