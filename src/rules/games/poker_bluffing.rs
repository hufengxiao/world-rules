//! 扑克诈唬博弈
//!
//! 扑克牌诈唬、看牌与牌桌心理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PokerBluffingRules,
    name: "扑克诈唬博弈",
    desc: "扑克牌诈唬、看牌与牌桌心理",
    origin: "国际",
    tags: ["游戏", "扑克", "诈唬", "博弈"]
}

impl PokerBluffingRules {
    /// 看牌手牌
    pub fn cards(&self) -> Vec<&'static str> {
        vec!["看清自己手牌", "心里估算大小", "按牌行下注", "不显牌型"]
    }

    /// 诈唬玩法
    pub fn bluff(&self) -> Vec<&'static str> {
        vec!["薄牌可虚张声势", "表情保持平常", "看对手反应", "不无度诈"]
    }

    /// 下注顺序
    pub fn betting(&self) -> Vec<&'static str> {
        vec!["按序跟注加注", "筹码结算准确", "开牌确定", "不耍赖犯规"]
    }

    /// 风度输赢
    pub fn grace(&self) -> Vec<&'static str> {
        vec!["赢不炫耀", "输不失态", "不沉赌上头", "小玩怡情"]
    }
}

impl Rule for PokerBluffingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("poker_bluff")
    }

    fn explain(&self) -> String {
        format!(
            "【扑克诈唬博弈】\n{}",
            [
                format!(
                    "看牌手牌：\\n{}",
                    self.cards()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "诈唬玩法：\\n{}",
                    self.bluff()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "下注顺序：\\n{}",
                    self.betting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风度输赢：\\n{}",
                    self.grace()
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
    fn test_pokerbluffingrules_basic() {
        let rules = PokerBluffingRules::new();
        assert_eq!(rules.metadata().name, "扑克诈唬博弈");
        assert!(!rules.cards().is_empty());
        assert!(!rules.bluff().is_empty());
        assert!(!rules.betting().is_empty());
        assert!(!rules.grace().is_empty());
    }

    #[test]
    fn test_pokerbluffingrules_validation() {
        let rules = PokerBluffingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("poker_bluff"));
    }

    #[test]
    fn test_pokerbluffingrules_explain() {
        let rules = PokerBluffingRules::new();
        let e = rules.explain();
        assert!(e.contains("看牌手牌"));
        assert!(e.contains("诈唬玩法"));
        assert!(e.contains("下注顺序"));
    }
}
