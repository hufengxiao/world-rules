//! 不等式数独
//!
//! 在数独基础上加入不等号提示的逻辑谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FutoshikiPuzzleRules,
    name: "不等式数独",
    desc: "在数独基础上加入不等号提示的逻辑谜题",
    origin: "日本",
    tags: ["解谜", "逻辑", "数独"]
}

impl FutoshikiPuzzleRules {
    /// 谜题布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "方格填入一至边长数",
            "常见五乘五网格",
            "格间标注不等号",
            "部分格预填数字",
        ]
    }

    /// 基本规则
    pub fn rules(&self) -> Vec<&'static str> {
        vec![
            "每行数字不重复",
            "每列数字不重复",
            "类似数独基本规则",
            "数字范围依网格大小",
        ]
    }

    /// 不等号规则
    pub fn inequality(&self) -> Vec<&'static str> {
        vec![
            "尖头指向较小数字",
            "开口指向较大数字",
            "相邻格须满足不等式",
            "无标记者不受约束",
        ]
    }

    /// 推理技巧
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "从最大最小数入手",
            "利用不等号排除",
            "结合行列唯一性",
            "逐步缩小候选范围",
        ]
    }
}

impl Rule for FutoshikiPuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("futoshiki_puzzle")
    }

    fn explain(&self) -> String {
        format!(
            "【不等式数独】\n{}",
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
                    "不等号规则：\\n{}",
                    self.inequality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "推理技巧：\\n{}",
                    self.techniques()
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
    fn test_futoshikipuzzlerules_basic() {
        let rules = FutoshikiPuzzleRules::new();
        assert_eq!(rules.metadata().name, "不等式数独");
        assert!(!rules.setup().is_empty());
        assert!(!rules.rules().is_empty());
        assert!(!rules.inequality().is_empty());
        assert!(!rules.techniques().is_empty());
    }

    #[test]
    fn test_futoshikipuzzlerules_validation() {
        let rules = FutoshikiPuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("futoshiki_puzzle"));
    }

    #[test]
    fn test_futoshikipuzzlerules_explain() {
        let rules = FutoshikiPuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("谜题布置"));
        assert!(e.contains("基本规则"));
        assert!(e.contains("不等号规则"));
    }
}
