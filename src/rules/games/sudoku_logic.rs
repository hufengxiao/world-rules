//! 数独解题逻辑
//!
//! 数独规则、排除法推理与解题技巧

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SudokuLogicRules,
    name: "数独解题逻辑",
    desc: "数独规则、排除法推理与解题技巧",
    origin: "国际",
    tags: ["游戏", "数独", "逻辑"]
}

impl SudokuLogicRules {
    /// 基本规则
    pub fn rule(&self) -> Vec<&'static str> {
        vec![
            "九宫格填1到9",
            "每行无重复数字",
            "每列不重复",
            "每宫也不重复",
        ]
    }

    /// 排除推理
    pub fn deduce(&self) -> Vec<&'static str> {
        vec![
            "根据已有数字排除",
            "看行列宫交叉",
            "唯一余数确定",
            "由易到难",
        ]
    }

    /// 检查验证
    pub fn verify(&self) -> Vec<&'static str> {
        vec!["填完核对规则", "不猜不硬试", "利用提示缓冲", "答案唯一"]
    }

    /// 解题耐心
    pub fn habit(&self) -> Vec<&'static str> {
        vec!["按部就班不躁", "多角度尝试", "记录候选", "享受逻辑乐趣"]
    }
}

impl Rule for SudokuLogicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("sudoku")
    }

    fn explain(&self) -> String {
        format!(
            "【数独解题逻辑】\n{}",
            [
                format!(
                    "基本规则：\\n{}",
                    self.rule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "排除推理：\\n{}",
                    self.deduce()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "检查验证：\\n{}",
                    self.verify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解题耐心：\\n{}",
                    self.habit()
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
    fn test_sudokulogicrules_basic() {
        let rules = SudokuLogicRules::new();
        assert_eq!(rules.metadata().name, "数独解题逻辑");
        assert!(!rules.rule().is_empty());
        assert!(!rules.deduce().is_empty());
        assert!(!rules.verify().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_sudokulogicrules_validation() {
        let rules = SudokuLogicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("sudoku"));
    }

    #[test]
    fn test_sudokulogicrules_explain() {
        let rules = SudokuLogicRules::new();
        let e = rules.explain();
        assert!(e.contains("基本规则"));
        assert!(e.contains("排除推理"));
        assert!(e.contains("检查验证"));
    }
}
