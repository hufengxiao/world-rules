//! 闲聊拉近话题
//!
//! 与初识或泛泛之交闲聊时找话题与拿捏分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CasualSmallTalkStartRules,
    name: "闲聊拉近话题",
    desc: "与初识或泛泛之交闲聊时找话题与拿捏分寸",
    origin: "中国",
    tags: ["社交", "闲聊", "话题", "沟通"]
}

impl CasualSmallTalkStartRules {
    /// 找话题
    pub fn topic(&self) -> Vec<&'static str> {
        vec!["聊天气近况", "谈共同喜好", "问对方兴趣", "适度分享自己"]
    }

    /// 展开分寸
    pub fn tact(&self) -> Vec<&'static str> {
        vec!["先易后深", "不打听隐私", "顺着对方接", "不冷场也不抢话"]
    }

    /// 保持轻松
    pub fn light(&self) -> Vec<&'static str> {
        vec!["幽默得体", "不说长道闲", "笑而不可艮", "保持友好氛围"]
    }

    /// 见好就收
    pub fn close(&self) -> Vec<&'static str> {
        vec!["不留恋尬聊", "适时道别", "给话头留余地", "下回还有话题"]
    }
}

impl Rule for CasualSmallTalkStartRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("small_talk")
    }

    fn explain(&self) -> String {
        format!(
            "【闲聊拉近话题】\n{}",
            [
                format!(
                    "找话题：\\n{}",
                    self.topic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "展开分寸：\\n{}",
                    self.tact()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保持轻松：\\n{}",
                    self.light()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "见好就收：\\n{}",
                    self.close()
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
    fn test_casualsmalltalkstartrules_basic() {
        let rules = CasualSmallTalkStartRules::new();
        assert_eq!(rules.metadata().name, "闲聊拉近话题");
        assert!(!rules.topic().is_empty());
        assert!(!rules.tact().is_empty());
        assert!(!rules.light().is_empty());
        assert!(!rules.close().is_empty());
    }

    #[test]
    fn test_casualsmalltalkstartrules_validation() {
        let rules = CasualSmallTalkStartRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("small_talk"));
    }

    #[test]
    fn test_casualsmalltalkstartrules_explain() {
        let rules = CasualSmallTalkStartRules::new();
        let e = rules.explain();
        assert!(e.contains("找话题"));
        assert!(e.contains("展开分寸"));
        assert!(e.contains("保持轻松"));
    }
}
