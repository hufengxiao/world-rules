//! 幻方填数
//!
//! 幻方行、列、对线数字和相等的谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MagicSquareFillRules,
    name: "幻方填数",
    desc: "幻方行、列、对线数字和相等的谜题",
    origin: "数学",
    tags: ["游戏", "幻方", "数学", "谜题"]
}

impl MagicSquareFillRules {
    /// 幻方定义
    pub fn definition(&self) -> Vec<&'static str> {
        vec![
            "方格排于数字",
            "每行每列各线数字和相等",
            "不重复不省略数",
            "常见三阶幻方",
        ]
    }

    /// 三阶填法
    pub fn three(&self) -> Vec<&'static str> {
        vec!["中间放五", "对角一二三位置", "飞码对称", "和均为十五"]
    }

    /// 奇数阶
    pub fn odd(&self) -> Vec<&'static str> {
        vec!["从中间行起", "超出折回", "遇格退下一格", "依序填充"]
    }

    /// 检验乐学
    pub fn verify(&self) -> Vec<&'static str> {
        vec!["计算各行检验", "调整再算", "锻炼算数", "乐在其中"]
    }
}

impl Rule for MagicSquareFillRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("magic_square")
    }

    fn explain(&self) -> String {
        format!(
            "【幻方填数】\n{}",
            [
                format!(
                    "幻方定义：\\n{}",
                    self.definition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "三阶填法：\\n{}",
                    self.three()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "奇数阶：\\n{}",
                    self.odd()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "检验乐学：\\n{}",
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
    fn test_magicsquarefillrules_basic() {
        let rules = MagicSquareFillRules::new();
        assert_eq!(rules.metadata().name, "幻方填数");
        assert!(!rules.definition().is_empty());
        assert!(!rules.three().is_empty());
        assert!(!rules.odd().is_empty());
        assert!(!rules.verify().is_empty());
    }

    #[test]
    fn test_magicsquarefillrules_validation() {
        let rules = MagicSquareFillRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("magic_square"));
    }

    #[test]
    fn test_magicsquarefillrules_explain() {
        let rules = MagicSquareFillRules::new();
        let e = rules.explain();
        assert!(e.contains("幻方定义"));
        assert!(e.contains("三阶填法"));
        assert!(e.contains("奇数阶"));
    }
}
