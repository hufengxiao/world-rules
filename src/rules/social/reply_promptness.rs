//! 及时回复礼仪
//!
//! 收到信息及时回应与忙碌时如何交代的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReplyPromptnessRules,
    name: "及时回复礼仪",
    desc: "收到信息及时回应与忙碌时如何交代的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "回复", "及时"]
}

impl ReplyPromptnessRules {
    /// 及时回应
    pub fn prompt(&self) -> Vec<&'static str> {
        vec![
            "重要消息尽早回复",
            "简短确认已收到",
            "暂忙可说明",
            "不无端沉默",
        ]
    }

    /// 务实表达
    pub fn express(&self) -> Vec<&'static str> {
        vec![
            "回复明确不回避",
            "需时间承办说明",
            "办成告知结果",
            "不敷衍虚假承诺",
        ]
    }

    /// 特殊情况
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "忙碌无法即回可稍后",
            "未覆盖信息补回",
            "遗漏解释不敷衍",
            "尊重双方时机",
        ]
    }

    /// 度与分寸
    pub fn balance(&self) -> Vec<&'static str> {
        vec![
            "不过度焦虑盯消息",
            "不被无意义打扰缠身",
            "重要消息优先",
            "体谅他人等待",
        ]
    }
}

impl Rule for ReplyPromptnessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("reply_prompt")
    }

    fn explain(&self) -> String {
        format!(
            "【及时回复礼仪】\n{}",
            [
                format!(
                    "及时回应：\\n{}",
                    self.prompt()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "务实表达：\\n{}",
                    self.express()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊情况：\\n{}",
                    self.special()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "度与分寸：\\n{}",
                    self.balance()
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
    fn test_replypromptnessrules_basic() {
        let rules = ReplyPromptnessRules::new();
        assert_eq!(rules.metadata().name, "及时回复礼仪");
        assert!(!rules.prompt().is_empty());
        assert!(!rules.express().is_empty());
        assert!(!rules.special().is_empty());
        assert!(!rules.balance().is_empty());
    }

    #[test]
    fn test_replypromptnessrules_validation() {
        let rules = ReplyPromptnessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("reply_prompt"));
    }

    #[test]
    fn test_replypromptnessrules_explain() {
        let rules = ReplyPromptnessRules::new();
        let e = rules.explain();
        assert!(e.contains("及时回应"));
        assert!(e.contains("务实表达"));
        assert!(e.contains("特殊情况"));
    }
}
