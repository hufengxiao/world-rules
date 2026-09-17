//! 接受批评与反馈
//!
//! 面对他人批评与反馈时的倾听、反思与成长

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AcceptingFeedbackRules,
    name: "接受批评与反馈",
    desc: "面对他人批评与反馈时的倾听、反思与成长",
    origin: "国际",
    tags: ["社交", "礼仪", "批评", "反馈", "成长"]
}

impl AcceptingFeedbackRules {
    /// 倾听态度
    pub fn listen(&self) -> Vec<&'static str> {
        vec![
            "保持开放不急于反驳",
            "专注听完对方表达",
            "示意理解对方的观点",
            "不打断或情绪反弹",
        ]
    }

    /// 理性回应
    pub fn respond(&self) -> Vec<&'static str> {
        vec![
            "先感谢对方的指正",
            "针对事情本身讨论",
            "表达自己的侧不辩解",
            "不过分自责或指责",
        ]
    }

    /// 反思改进
    pub fn improve(&self) -> Vec<&'static str> {
        vec![
            "客观评估反馈的价值",
            "重要的是吸取有用部分",
            "制定可行的改进",
            "持续实践并反馈",
        ]
    }

    /// 关系维护
    pub fn relationship(&self) -> Vec<&'static str> {
        vec![
            "不记恨给予反馈者",
            "把反馈视为进步机会",
            "后续表达进展",
            "尊重不同意见",
        ]
    }
}

impl Rule for AcceptingFeedbackRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("feedback")
    }

    fn explain(&self) -> String {
        format!(
            "【接受批评与反馈】\n{}",
            [
                format!(
                    "倾听态度：\\n{}",
                    self.listen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "理性回应：\\n{}",
                    self.respond()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "反思改进：\\n{}",
                    self.improve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "关系维护：\\n{}",
                    self.relationship()
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
    fn test_acceptingfeedbackrules_basic() {
        let rules = AcceptingFeedbackRules::new();
        assert_eq!(rules.metadata().name, "接受批评与反馈");
        assert!(!rules.listen().is_empty());
        assert!(!rules.respond().is_empty());
        assert!(!rules.improve().is_empty());
        assert!(!rules.relationship().is_empty());
    }

    #[test]
    fn test_acceptingfeedbackrules_validation() {
        let rules = AcceptingFeedbackRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("feedback"));
    }

    #[test]
    fn test_acceptingfeedbackrules_explain() {
        let rules = AcceptingFeedbackRules::new();
        let e = rules.explain();
        assert!(e.contains("倾听态度"));
        assert!(e.contains("理性回应"));
        assert!(e.contains("反思改进"));
    }
}
