//! 骰子游戏规则
//!
//! 骰子点数、轮流投掷与游戏公平

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DiceGameQuantumRules,
    name: "骰子游戏规则",
    desc: "骰子点数、轮流投掷与游戏公平",
    origin: "国际",
    tags: ["游戏", "骰子", "随机"]
}

impl DiceGameQuantumRules {
    /// 掷骰规则
    pub fn throw(&self) -> Vec<&'static str> {
        vec!["公平掷骰", "一次掷定", "不重掷赖皮", "按协商约定"]
    }

    /// 点数判断
    pub fn judge(&self) -> Vec<&'static str> {
        vec!["计总和比点", "大小单双玩法", "按约定规则", "结果辨明"]
    }

    /// 轮流顺序
    pub fn turn(&self) -> Vec<&'static str> {
        vec!["轮流掷骰", "顺次序进行", "不抢不催", "公平交替"]
    }

    /// 游戏伦理
    pub fn ethics(&self) -> Vec<&'static str> {
        vec!["靠运不靠诈", "不偷换骰子", "愉快玩耍", "输赢看开"]
    }
}

impl Rule for DiceGameQuantumRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("dice_game")
    }

    fn explain(&self) -> String {
        format!(
            "【骰子游戏规则】\n{}",
            [
                format!(
                    "掷骰规则：\\n{}",
                    self.throw()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "点数判断：\\n{}",
                    self.judge()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "轮流顺序：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "游戏伦理：\\n{}",
                    self.ethics()
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
    fn test_dicegamequantumrules_basic() {
        let rules = DiceGameQuantumRules::new();
        assert_eq!(rules.metadata().name, "骰子游戏规则");
        assert!(!rules.throw().is_empty());
        assert!(!rules.judge().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.ethics().is_empty());
    }

    #[test]
    fn test_dicegamequantumrules_validation() {
        let rules = DiceGameQuantumRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("dice_game"));
    }

    #[test]
    fn test_dicegamequantumrules_explain() {
        let rules = DiceGameQuantumRules::new();
        let e = rules.explain();
        assert!(e.contains("掷骰规则"));
        assert!(e.contains("点数判断"));
        assert!(e.contains("轮流顺序"));
    }
}
