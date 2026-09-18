//! 八皇后问题
//!
//! 八皇后互不攻击的摆法推理谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NQueensPuzzleRules,
    name: "八皇后问题",
    desc: "八皇后互不攻击的摆法推理谜题",
    origin: "数学",
    tags: ["游戏", "皇后", "棋盘", "谜题"]
}

impl NQueensPuzzleRules {
    /// 题目设定
    pub fn setup(&self) -> Vec<&'static str> {
        vec!["棋盘八行八列", "放置八枚皇后", "互不攻击", "各有解法"]
    }

    /// 互不攻击
    pub fn rule(&self) -> Vec<&'static str> {
        vec!["同行不能两后", "同列不行", "斜线不可相对", "全盘安全"]
    }

    /// 求解思路
    pub fn solve(&self) -> Vec<&'static str> {
        vec!["逐行放置", "排除冲突位置", "试探推演", "回退调整"]
    }

    /// 验证欣赏
    pub fn verify(&self) -> Vec<&'static str> {
        vec!["放完再检查", "重整再试", "锻炼逻辑", "乐在解题"]
    }
}

impl Rule for NQueensPuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("n_queens")
    }

    fn explain(&self) -> String {
        format!(
            "【八皇后问题】\n{}",
            [
                format!(
                    "题目设定：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "互不攻击：\\n{}",
                    self.rule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "求解思路：\\n{}",
                    self.solve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "验证欣赏：\\n{}",
                    self.verify()
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
    fn test_nqueenspuzzlerules_basic() {
        let rules = NQueensPuzzleRules::new();
        assert_eq!(rules.metadata().name, "八皇后问题");
        assert!(!rules.setup().is_empty());
        assert!(!rules.rule().is_empty());
        assert!(!rules.solve().is_empty());
        assert!(!rules.verify().is_empty());
    }

    #[test]
    fn test_nqueenspuzzlerules_validation() {
        let rules = NQueensPuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("n_queens"));
    }

    #[test]
    fn test_nqueenspuzzlerules_explain() {
        let rules = NQueensPuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("题目设定"));
        assert!(e.contains("互不攻击"));
        assert!(e.contains("求解思路"));
    }
}
