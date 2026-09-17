//! 吃墩牌类基础
//!
//! 桥牌等吃墩游戏的叫牌、跟牌与赢墩规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TrickTakingCardsRules,
    name: "吃墩牌类基础",
    desc: "桥牌等吃墩游戏的叫牌、跟牌与赢墩规则",
    origin: "国际",
    tags: ["游戏", "吃墩", "桥牌", "纸牌"]
}

impl TrickTakingCardsRules {
    /// 领牌跟牌
    pub fn lead(&self) -> Vec<&'static str> {
        vec![
            "首出者领花色",
            "其他人须跟同花色",
            "无同花色可垫牌",
            "可出王牌抢墩",
        ]
    }

    /// 赢墩判定
    pub fn win_trick(&self) -> Vec<&'static str> {
        vec![
            "牌面最大者赢墩",
            "王牌通常是最大",
            "同花色比大小",
            "领主花色须优先",
        ]
    }

    /// 叫牌约定
    pub fn bidding(&self) -> Vec<&'static str> {
        vec![
            "叫牌定主与目标",
            "合约规定赢墩数",
            "按叫牌顺序加码",
            "默契配合记分",
        ]
    }

    /// 策略配合
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "计算庄家余牌",
            "不说暗语交流",
            "伙伴间默契出牌",
            "遵守出牌节奏",
        ]
    }
}

impl Rule for TrickTakingCardsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("trick_taking")
    }

    fn explain(&self) -> String {
        format!(
            "【吃墩牌类基础】\n{}",
            [
                format!(
                    "领牌跟牌：\\n{}",
                    self.lead()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赢墩判定：\\n{}",
                    self.win_trick()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "叫牌约定：\\n{}",
                    self.bidding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "策略配合：\\n{}",
                    self.strategy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_tricktakingcardsrules_basic() {
        let rules = TrickTakingCardsRules::new();
        assert_eq!(rules.metadata().name, "吃墩牌类基础");
        assert!(!rules.lead().is_empty());
        assert!(!rules.win_trick().is_empty());
        assert!(!rules.bidding().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_tricktakingcardsrules_validation() {
        let rules = TrickTakingCardsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("trick_taking"));
    }

    #[test]
    fn test_tricktakingcardsrules_explain() {
        let rules = TrickTakingCardsRules::new();
        let e = rules.explain();
        assert!(e.contains("领牌跟牌"));
        assert!(e.contains("赢墩判定"));
        assert!(e.contains("叫牌约定"));
    }
}
