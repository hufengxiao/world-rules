//! 算24点
//!
//! 用四张牌四则运算凑24的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: Game24ArithmeticDeriveRules,
    name: "算24点",
    desc: "用四张牌四则运算凑24的规则",
    origin: "中国",
    tags: ["游戏", "24点", "算术", "益智"]
}

impl Game24ArithmeticDeriveRules {
    /// 出牌
    pub fn cards(&self) -> Vec<&'static str> {
        vec!["随机翻四张牌", "用加减乘除", "每张用一次", "组合括号"]
    }

    /// 运算成24
    pub fn calculate(&self) -> Vec<&'static str> {
        vec![
            "结果恰好为24",
            "可用加乘减除",
            "合理运用括号",
            "凑数运算到24",
        ]
    }

    /// 抢答规则
    pub fn answer(&self) -> Vec<&'static str> {
        vec!["先算出者亮法", "展示算式", "验对则得分", "玩得开心"]
    }

    /// 无解处理
    pub fn no_solution(&self) -> Vec<&'static str> {
        vec!["实在无解换牌", "公认无解可过", "锻炼心算", "趣味益智"]
    }
}

impl Rule for Game24ArithmeticDeriveRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("game_24")
    }

    fn explain(&self) -> String {
        format!(
            "【算24点】\n{}",
            [
                format!(
                    "出牌：\\n{}",
                    self.cards()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运算成24：\\n{}",
                    self.calculate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抢答规则：\\n{}",
                    self.answer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "无解处理：\\n{}",
                    self.no_solution()
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
    fn test_game24arithmeticderiverules_basic() {
        let rules = Game24ArithmeticDeriveRules::new();
        assert_eq!(rules.metadata().name, "算24点");
        assert!(!rules.cards().is_empty());
        assert!(!rules.calculate().is_empty());
        assert!(!rules.answer().is_empty());
        assert!(!rules.no_solution().is_empty());
    }

    #[test]
    fn test_game24arithmeticderiverules_validation() {
        let rules = Game24ArithmeticDeriveRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("game_24"));
    }

    #[test]
    fn test_game24arithmeticderiverules_explain() {
        let rules = Game24ArithmeticDeriveRules::new();
        let e = rules.explain();
        assert!(e.contains("出牌"));
        assert!(e.contains("运算成24"));
        assert!(e.contains("抢答规则"));
    }
}
