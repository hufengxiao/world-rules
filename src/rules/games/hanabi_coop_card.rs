//! 花火合作牌
//!
//! 花火游戏中基于信息提示的合作出牌规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HanabiCoopCardRules,
    name: "花火合作牌",
    desc: "花火游戏中基于信息提示的合作出牌规则",
    origin: "日本",
    tags: ["游戏", "卡牌", "合作", "桌游"]
}

impl HanabiCoopCardRules {
    /// 游戏目标
    pub fn goal(&self) -> Vec<&'static str> {
        vec!["合作放烟火", "按花色点数排列", "从1清理到5", "各色独立堆叠"]
    }

    /// 信息提示
    pub fn hint(&self) -> Vec<&'static str> {
        vec![
            "不得直接看手牌",
            "提示牌数量色",
            "他出牌信息共享",
            "有限提示数",
        ]
    }

    /// 出牌纪律
    pub fn play(&self) -> Vec<&'static str> {
        vec![
            "按提示出对应牌",
            "按序接续牌列",
            "错误出牌扣分",
            "错过实际牌会用错",
        ]
    }

    /// 协作默契
    pub fn cooperate(&self) -> Vec<&'static str> {
        vec![
            "记忆各家提示",
            "以剩余提示暗示意",
            "留牌待后需要",
            "齐心协力共取胜",
        ]
    }
}

impl Rule for HanabiCoopCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("hanabi_card")
    }

    fn explain(&self) -> String {
        format!(
            "【花火合作牌】\n{}",
            [
                format!(
                    "游戏目标：\\n{}",
                    self.goal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "信息提示：\\n{}",
                    self.hint()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "出牌纪律：\\n{}",
                    self.play()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "协作默契：\\n{}",
                    self.cooperate()
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
    fn test_hanabicoopcardrules_basic() {
        let rules = HanabiCoopCardRules::new();
        assert_eq!(rules.metadata().name, "花火合作牌");
        assert!(!rules.goal().is_empty());
        assert!(!rules.hint().is_empty());
        assert!(!rules.play().is_empty());
        assert!(!rules.cooperate().is_empty());
    }

    #[test]
    fn test_hanabicoopcardrules_validation() {
        let rules = HanabiCoopCardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("hanabi_card"));
    }

    #[test]
    fn test_hanabicoopcardrules_explain() {
        let rules = HanabiCoopCardRules::new();
        let e = rules.explain();
        assert!(e.contains("游戏目标"));
        assert!(e.contains("信息提示"));
        assert!(e.contains("出牌纪律"));
    }
}
