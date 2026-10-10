//! 一人格
//!
//! 通过涂黑格子消除重复数字的逻辑谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HitoriPuzzleRules,
    name: "一人格",
    desc: "通过涂黑格子消除重复数字的逻辑谜题",
    origin: "日本",
    tags: ["解谜", "逻辑", "数独"]
}

impl HitoriPuzzleRules {
    /// 谜题布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "方格内填入数字",
            "多为五乘五或更大",
            "数字可有重复",
            "部分格子需涂黑",
        ]
    }

    /// 涂黑规则
    pub fn rules(&self) -> Vec<&'static str> {
        vec![
            "每行每列数字不可重复",
            "重复者须涂黑其一",
            "涂黑格不可相邻",
            "未涂黑格须连成一片",
        ]
    }

    /// 推理技巧
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "相邻同数必涂其一",
            "被夹同数中间留白",
            "黑白交替避免相连",
            "逐步推导唯一解",
        ]
    }

    /// 解答验证
    pub fn verification(&self) -> Vec<&'static str> {
        vec![
            "行内无重复数字",
            "列内无重复数字",
            "涂黑格互不相邻",
            "留白区连通成一体",
        ]
    }
}

impl Rule for HitoriPuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("hitori_puzzle")
    }

    fn explain(&self) -> String {
        format!(
            "【一人格】\n{}",
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
                    "涂黑规则：\\n{}",
                    self.rules()
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
    fn test_hitoripuzzlerules_basic() {
        let rules = HitoriPuzzleRules::new();
        assert_eq!(rules.metadata().name, "一人格");
        assert!(!rules.setup().is_empty());
        assert!(!rules.rules().is_empty());
        assert!(!rules.techniques().is_empty());
        assert!(!rules.verification().is_empty());
    }

    #[test]
    fn test_hitoripuzzlerules_validation() {
        let rules = HitoriPuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("hitori_puzzle"));
    }

    #[test]
    fn test_hitoripuzzlerules_explain() {
        let rules = HitoriPuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("谜题布置"));
        assert!(e.contains("涂黑规则"));
        assert!(e.contains("推理技巧"));
    }
}
