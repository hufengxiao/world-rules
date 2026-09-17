//! 工作邮件礼仪
//!
//! 工作邮件的主题、格式与回复礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WorkEmailProtocolRules,
    name: "工作邮件礼仪",
    desc: "工作邮件的主题、格式与回复礼仪",
    origin: "国际",
    tags: ["职场", "邮件", "礼仪"]
}

impl WorkEmailProtocolRules {
    /// 主题清楚
    pub fn subject(&self) -> Vec<&'static str> {
        vec!["主题简洁达意", "写明事由", "紧急标注紧急", "不空白主题"]
    }

    /// 内容规范
    pub fn content(&self) -> Vec<&'static str> {
        vec!["称呼对象得体", "正文有条理", "表达清楚", "结尾落款礼貌"]
    }

    /// 及时回复
    pub fn reply(&self) -> Vec<&'static str> {
        vec!["收到邮件及时回", "询问细节答清", "不能及时说明", "礼貌委婉"]
    }

    /// 转发分寸
    pub fn forward(&self) -> Vec<&'static str> {
        vec!["抄送范围恰当", "不随便转发他人", "附件命名清楚", "保护隐私"]
    }
}

impl Rule for WorkEmailProtocolRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("work_email")
    }

    fn explain(&self) -> String {
        format!(
            "【工作邮件礼仪】\n{}",
            [
                format!(
                    "主题清楚：\\n{}",
                    self.subject()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "内容规范：\\n{}",
                    self.content()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "及时回复：\\n{}",
                    self.reply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "转发分寸：\\n{}",
                    self.forward()
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
    fn test_workemailprotocolrules_basic() {
        let rules = WorkEmailProtocolRules::new();
        assert_eq!(rules.metadata().name, "工作邮件礼仪");
        assert!(!rules.subject().is_empty());
        assert!(!rules.content().is_empty());
        assert!(!rules.reply().is_empty());
        assert!(!rules.forward().is_empty());
    }

    #[test]
    fn test_workemailprotocolrules_validation() {
        let rules = WorkEmailProtocolRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("work_email"));
    }

    #[test]
    fn test_workemailprotocolrules_explain() {
        let rules = WorkEmailProtocolRules::new();
        let e = rules.explain();
        assert!(e.contains("主题清楚"));
        assert!(e.contains("内容规范"));
        assert!(e.contains("及时回复"));
    }
}
