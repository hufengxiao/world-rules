//! 算术指令游戏
//!
//! 按指令加减运算的益智与心算玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ArithmeticCommandGameRules,
    name: "算术指令游戏",
    desc: "按指令加减运算的益智与心算玩法",
    origin: "数学",
    tags: ["游戏", "算术", "指令", "益智"]
}

impl ArithmeticCommandGameRules {
    /// 指令设定
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "给一串加减乘除指令",
            "从初始数开始",
            "依次计算",
            "得出最终值",
        ]
    }

    /// 执行规范
    pub fn execute(&self) -> Vec<&'static str> {
        vec!["按顺序计算", "看清符号", "乘除优先算", "依序执行"]
    }

    /// 心算核对
    pub fn check(&self) -> Vec<&'static str> {
        vec!["算完复核", "逆向验算", "结果一致", "锻炼心算"]
    }

    /// 对练挑战
    pub fn challenge(&self) -> Vec<&'static str> {
        vec!["多人出题互答", "限时作答", "比谁准而快", "寓教于乐"]
    }
}

impl Rule for ArithmeticCommandGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("arithmetic_command")
    }

    fn explain(&self) -> String {
        format!(
            "【算术指令游戏】\n{}",
            [
                format!(
                    "指令设定：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "执行规范：\\n{}",
                    self.execute()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "心算核对：\\n{}",
                    self.check()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "对练挑战：\\n{}",
                    self.challenge()
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
    fn test_arithmeticcommandgamerules_basic() {
        let rules = ArithmeticCommandGameRules::new();
        assert_eq!(rules.metadata().name, "算术指令游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.execute().is_empty());
        assert!(!rules.check().is_empty());
        assert!(!rules.challenge().is_empty());
    }

    #[test]
    fn test_arithmeticcommandgamerules_validation() {
        let rules = ArithmeticCommandGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("arithmetic_command"));
    }

    #[test]
    fn test_arithmeticcommandgamerules_explain() {
        let rules = ArithmeticCommandGameRules::new();
        let e = rules.explain();
        assert!(e.contains("指令设定"));
        assert!(e.contains("执行规范"));
        assert!(e.contains("心算核对"));
    }
}
