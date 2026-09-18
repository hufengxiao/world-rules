//! 杀手数独
//!
//! 杀手数独并入笼和数求和推理的玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KillerSudokuSumRules,
    name: "杀手数独",
    desc: "杀手数独并入笼和数求和推理的玩法",
    origin: "国际",
    tags: ["游戏", "数独", "杀手", "推理"]
}

impl KillerSudokuSumRules {
    /// 基本规则
    pub fn sum(&self) -> Vec<&'static str> {
        vec![
            "行列宫填一到九不重复",
            "虚线笼内数字和",
            "小格标和为笼和",
            "符合和即合法",
        ]
    }

    /// 笼和推理
    pub fn infer(&self) -> Vec<&'static str> {
        vec!["小笼组合有限", "唯一组合优先填", "逐格排除", "交叉判断"]
    }

    /// 行列宫互推
    pub fn grid(&self) -> Vec<&'static str> {
        vec!["结合行列宫", "锁定候选", "由简到繁", "逐步推进"]
    }

    /// 享受推理
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["解题耐心", "失败重试", "锻炼逻辑思维", "乐在其中"]
    }
}

impl Rule for KillerSudokuSumRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("killer_sudoku")
    }

    fn explain(&self) -> String {
        format!(
            "【杀手数独】\n{}",
            [
                format!(
                    "基本规则：\\n{}",
                    self.sum()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "笼和推理：\\n{}",
                    self.infer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行列宫互推：\\n{}",
                    self.grid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "享受推理：\\n{}",
                    self.fun()
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
    fn test_killersudokusumrules_basic() {
        let rules = KillerSudokuSumRules::new();
        assert_eq!(rules.metadata().name, "杀手数独");
        assert!(!rules.sum().is_empty());
        assert!(!rules.infer().is_empty());
        assert!(!rules.grid().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_killersudokusumrules_validation() {
        let rules = KillerSudokuSumRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("killer_sudoku"));
    }

    #[test]
    fn test_killersudokusumrules_explain() {
        let rules = KillerSudokuSumRules::new();
        let e = rules.explain();
        assert!(e.contains("基本规则"));
        assert!(e.contains("笼和推理"));
        assert!(e.contains("行列宫互推"));
    }
}
