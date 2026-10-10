//! 贤者格
//!
//! 结合算术运算与数独规则的逻辑谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KenkenPuzzleRules,
    name: "贤者格",
    desc: "结合算术运算与数独规则的逻辑谜题",
    origin: "日本",
    tags: ["解谜", "逻辑", "算术"]
}

impl KenkenPuzzleRules {
    /// 谜题布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "网格多为四乘四至六乘六",
            "粗线框分出若干组",
            "每框标注目标数与运算",
            "填入一至网格边长数",
        ]
    }

    /// 基本规则
    pub fn rules(&self) -> Vec<&'static str> {
        vec![
            "每行数字不重复",
            "每列数字不重复",
            "类似数独规则",
            "数字范围依网格大小",
        ]
    }

    /// 运算规则
    pub fn arithmetic(&self) -> Vec<&'static str> {
        vec![
            "框内数字按运算得目标数",
            "加减乘除四种运算",
            "单格框直接填数字",
            "顺序不限只求结果",
        ]
    }

    /// 解答验证
    pub fn verification(&self) -> Vec<&'static str> {
        vec![
            "每行每列无重复",
            "各框运算结果正确",
            "数字在有效范围内",
            "唯一解为标准",
        ]
    }
}

impl Rule for KenkenPuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("kenken_puzzle")
    }

    fn explain(&self) -> String {
        format!(
            "【贤者格】\n{}",
            [
                format!(
                    "谜题布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "基本规则：\\n{}",
                    self.rules()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运算规则：\\n{}",
                    self.arithmetic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解答验证：\\n{}",
                    self.verification()
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
    fn test_kenkenpuzzlerules_basic() {
        let rules = KenkenPuzzleRules::new();
        assert_eq!(rules.metadata().name, "贤者格");
        assert!(!rules.setup().is_empty());
        assert!(!rules.rules().is_empty());
        assert!(!rules.arithmetic().is_empty());
        assert!(!rules.verification().is_empty());
    }

    #[test]
    fn test_kenkenpuzzlerules_validation() {
        let rules = KenkenPuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("kenken_puzzle"));
    }

    #[test]
    fn test_kenkenpuzzlerules_explain() {
        let rules = KenkenPuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("谜题布置"));
        assert!(e.contains("基本规则"));
        assert!(e.contains("运算规则"));
    }
}
