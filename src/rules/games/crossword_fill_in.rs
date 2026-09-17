//! 填字游戏
//!
//! 纵横填字的规则、提示与解题技巧

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CrosswordFillInRules,
    name: "填字游戏",
    desc: "纵横填字的规则、提示与解题技巧",
    origin: "国际",
    tags: ["游戏", "填字", "字谜", "益智"]
}

impl CrosswordFillInRules {
    /// 规则
    pub fn rule(&self) -> Vec<&'static str> {
        vec!["横竖格交叉填字", "每格一字", "提示给线索", "答案相互印证"]
    }

    /// 填法
    pub fn filling(&self) -> Vec<&'static str> {
        vec![
            "先填有把握的",
            "借交叉字推断",
            "多猜易错复核",
            "由浅容易入手",
        ]
    }

    /// 提示运用
    pub fn clues(&self) -> Vec<&'static str> {
        vec!["细读提示含义", "注意词性字数", "查字典辅助", "不急于求成"]
    }

    /// 完成乐趣
    pub fn enjoy(&self) -> Vec<&'static str> {
        vec!["填完有成就感", "可多查阅积累", "人多合作更快", "乐在锻炼"]
    }
}

impl Rule for CrosswordFillInRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("crossword")
    }

    fn explain(&self) -> String {
        format!(
            "【填字游戏】\n{}",
            [
                format!(
                    "规则：\\n{}",
                    self.rule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "填法：\\n{}",
                    self.filling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "提示运用：\\n{}",
                    self.clues()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "完成乐趣：\\n{}",
                    self.enjoy()
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
    fn test_crosswordfillinrules_basic() {
        let rules = CrosswordFillInRules::new();
        assert_eq!(rules.metadata().name, "填字游戏");
        assert!(!rules.rule().is_empty());
        assert!(!rules.filling().is_empty());
        assert!(!rules.clues().is_empty());
        assert!(!rules.enjoy().is_empty());
    }

    #[test]
    fn test_crosswordfillinrules_validation() {
        let rules = CrosswordFillInRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("crossword"));
    }

    #[test]
    fn test_crosswordfillinrules_explain() {
        let rules = CrosswordFillInRules::new();
        let e = rules.explain();
        assert!(e.contains("规则"));
        assert!(e.contains("填法"));
        assert!(e.contains("提示运用"));
    }
}
