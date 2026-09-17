//! 猜画你画我猜
//!
//! 你画我猜、比划猜词的配合与规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CharadesDrawingGuessRules,
    name: "猜画你画我猜",
    desc: "你画我猜、比划猜词的配合与规则",
    origin: "中国",
    tags: ["游戏", "你画我猜", "猜词", "益智"]
}

impl CharadesDrawingGuessRules {
    /// 选题
    pub fn topics(&self) -> Vec<&'static str> {
        vec!["选常见易画词", "难易可调", "主题分组", "限制答题时间"]
    }

    /// 比划规则
    pub fn acting(&self) -> Vec<&'static str> {
        vec!["只能用动作比划", "不能说话拼音", "可拆字可示意", "不写答案"]
    }

    /// 猜答技巧
    pub fn guessing(&self) -> Vec<&'static str> {
        vec!["由粗到细猜", "追问个别字", "配合默契", "说话提示要少"]
    }

    /// 计分公平
    pub fn scoring(&self) -> Vec<&'static str> {
        vec!["限时猜中得分", "轮流上场", "分数公开", "愉快为主"]
    }
}

impl Rule for CharadesDrawingGuessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("charades")
    }

    fn explain(&self) -> String {
        format!(
            "【猜画你画我猜】\n{}",
            [
                format!(
                    "选题：\\n{}",
                    self.topics()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "比划规则：\\n{}",
                    self.acting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "猜答技巧：\\n{}",
                    self.guessing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分公平：\\n{}",
                    self.scoring()
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
    fn test_charadesdrawingguessrules_basic() {
        let rules = CharadesDrawingGuessRules::new();
        assert_eq!(rules.metadata().name, "猜画你画我猜");
        assert!(!rules.topics().is_empty());
        assert!(!rules.acting().is_empty());
        assert!(!rules.guessing().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_charadesdrawingguessrules_validation() {
        let rules = CharadesDrawingGuessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("charades"));
    }

    #[test]
    fn test_charadesdrawingguessrules_explain() {
        let rules = CharadesDrawingGuessRules::new();
        let e = rules.explain();
        assert!(e.contains("选题"));
        assert!(e.contains("比划规则"));
        assert!(e.contains("猜答技巧"));
    }
}
