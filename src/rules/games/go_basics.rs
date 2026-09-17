//! 围棋基本规则
//!
//! 围棋落子、气、提子与目数的基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GoBasicsRules,
    name: "围棋基本规则",
    desc: "围棋落子、气、提子与目数的基础规则",
    origin: "中国",
    tags: ["游戏", "围棋", "棋类"]
}

impl GoBasicsRules {
    /// 棋盘落子
    pub fn placing(&self) -> Vec<&'static str> {
        vec!["黑先白后落子", "落子于交叉点", "落子不可悔", "交替行棋"]
    }

    /// 气与提子
    pub fn liberties(&self) -> Vec<&'static str> {
        vec![
            "邻接空点是气",
            "无气之子被提",
            "禁止自杀落点",
            "围吃判断准确",
        ]
    }

    /// 目数与战斗
    pub fn capture(&self) -> Vec<&'static str> {
        vec!["围空多得目", "战斗靠连接与气", "打劫有规则", "兼顾厚与空"]
    }

    /// 行棋风范
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["尊重对手思考", "不悔子争闹", "复盘学习", "礼弈品格"]
    }
}

impl Rule for GoBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_basics")
    }

    fn explain(&self) -> String {
        format!(
            "【围棋基本规则】\n{}",
            [
                format!(
                    "棋盘落子：\\n{}",
                    self.placing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "气与提子：\\n{}",
                    self.liberties()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "目数与战斗：\\n{}",
                    self.capture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行棋风范：\\n{}",
                    self.manner()
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
    fn test_gobasicsrules_basic() {
        let rules = GoBasicsRules::new();
        assert_eq!(rules.metadata().name, "围棋基本规则");
        assert!(!rules.placing().is_empty());
        assert!(!rules.liberties().is_empty());
        assert!(!rules.capture().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_gobasicsrules_validation() {
        let rules = GoBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("go_basics"));
    }

    #[test]
    fn test_gobasicsrules_explain() {
        let rules = GoBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘落子"));
        assert!(e.contains("气与提子"));
        assert!(e.contains("目数与战斗"));
    }
}
