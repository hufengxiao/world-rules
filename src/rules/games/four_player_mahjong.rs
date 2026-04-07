//! 四人麻将规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 四人麻将规则
pub struct FourPlayerMahjongRules {
    metadata: RuleMetadata,
}

impl FourPlayerMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "四人麻将规则",
                "四人麻将通用规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "麻将".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "四人围坐，东南西北各一方",
            "使用136张牌(不含花牌)",
            "每人13张手牌",
            "庄家起手14张",
            "东家为庄家开始",
        ]
    }

    /// 游戏流程
    pub fn game_flow(&self) -> Vec<&'static str> {
        vec![
            "庄家出牌开始",
            "按逆时针轮流",
            "可以吃、碰、杠",
            "胡牌则本局结束",
            "胡牌者做庄",
        ]
    }

    /// 胡牌条件
    pub fn winning_conditions(&self) -> Vec<&'static str> {
        vec![
            "4组牌+1对将(标准型)",
            "7对子",
            "十三幺",
            "必须满足起胡番数",
        ]
    }

    /// 结算规则
    pub fn settlement_rules(&self) -> Vec<&'static str> {
        vec![
            "自摸: 三家支付",
            "点炮: 点炮者支付",
            "庄家胡牌: 番数翻倍",
            "连庄: 庄家连续坐庄",
        ]
    }
}

impl Default for FourPlayerMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FourPlayerMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("four_player_mahjong")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【四人麻将规则】\n\n\
            基本设置:\n{}\n\n\
            游戏流程:\n{}\n\n\
            胡牌条件:\n{}\n\n\
            结算规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.game_flow().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.winning_conditions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.settlement_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_player_mahjong_rules() {
        let rules = FourPlayerMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}