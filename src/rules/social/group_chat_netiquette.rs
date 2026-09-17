//! 群聊礼仪
//!
//! 微信群聊的信息适量、有序与不打扰礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GroupChatNetiquetteRules,
    name: "群聊礼仪",
    desc: "微信群聊的信息适量、有序与不打扰礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "群聊", "信息"]
}

impl GroupChatNetiquetteRules {
    /// 发帖适度
    pub fn post(&self) -> Vec<&'static str> {
        vec![
            "不发无关刷屏",
            "一次只说要点",
            "避免连发多图",
            "不传播不实信息",
        ]
    }

    /// 有序交流
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "回复相关对话",
            "避免歪楼偏题",
            "有序讨论不争吵",
            "尊重他人意见",
        ]
    }

    /// 成员尊重
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "不@骚扰无关人",
            "不发隐私内容",
            "不贬低讥笑他人",
            "友善互动",
        ]
    }

    /// 功能规范
    pub fn function(&self) -> Vec<&'static str> {
        vec![
            "公众号提醒适度",
            "待改事项确认",
            "退群得体说明",
            "维护群内秩序",
        ]
    }
}

impl Rule for GroupChatNetiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("group_chat")
    }

    fn explain(&self) -> String {
        format!(
            "【群聊礼仪】\n{}",
            [
                format!(
                    "发帖适度：\\n{}",
                    self.post()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "有序交流：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "成员尊重：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "功能规范：\\n{}",
                    self.function()
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
    fn test_groupchatnetiquetterules_basic() {
        let rules = GroupChatNetiquetteRules::new();
        assert_eq!(rules.metadata().name, "群聊礼仪");
        assert!(!rules.post().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.function().is_empty());
    }

    #[test]
    fn test_groupchatnetiquetterules_validation() {
        let rules = GroupChatNetiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("group_chat"));
    }

    #[test]
    fn test_groupchatnetiquetterules_explain() {
        let rules = GroupChatNetiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("发帖适度"));
        assert!(e.contains("有序交流"));
        assert!(e.contains("成员尊重"));
    }
}
