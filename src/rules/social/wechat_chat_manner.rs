//! 微信聊天礼仪
//!
//! 微信通讯的文字、问候与及时回复礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WechatChatMannerRules,
    name: "微信聊天礼仪",
    desc: "微信通讯的文字、问候与及时回复礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "聊天", "即时通讯"]
}

impl WechatChatMannerRules {
    /// 开场得体
    pub fn greet(&self) -> Vec<&'static str> {
        vec![
            "初聊先问候",
            "称呼恰当",
            "说明来意清楚",
            "避免一上来连发语音",
        ]
    }

    /// 文字表达
    pub fn text(&self) -> Vec<&'static str> {
        vec![
            "表达清晰不简短生硬",
            "恰当使用标点",
            "不群发骚扰",
            "不随意修改他人消息",
        ]
    }

    /// 语音使用
    pub fn voice(&self) -> Vec<&'static str> {
        vec![
            "重要信息用文字",
            "语音适中不连发",
            "考虑对方场景",
            "不深夜打扰",
        ]
    }

    /// 回复尺度
    pub fn reply(&self) -> Vec<&'static str> {
        vec![
            "看到及时回复",
            "缺席说明原因",
            "收到通知恰当回应",
            "不敷衍冷漠",
        ]
    }
}

impl Rule for WechatChatMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wechat")
    }

    fn explain(&self) -> String {
        format!(
            "【微信聊天礼仪】\n{}",
            [
                format!(
                    "开场得体：\\n{}",
                    self.greet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "文字表达：\\n{}",
                    self.text()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "语音使用：\\n{}",
                    self.voice()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回复尺度：\\n{}",
                    self.reply()
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
    fn test_wechatchatmannerrules_basic() {
        let rules = WechatChatMannerRules::new();
        assert_eq!(rules.metadata().name, "微信聊天礼仪");
        assert!(!rules.greet().is_empty());
        assert!(!rules.text().is_empty());
        assert!(!rules.voice().is_empty());
        assert!(!rules.reply().is_empty());
    }

    #[test]
    fn test_wechatchatmannerrules_validation() {
        let rules = WechatChatMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("wechat"));
    }

    #[test]
    fn test_wechatchatmannerrules_explain() {
        let rules = WechatChatMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("开场得体"));
        assert!(e.contains("文字表达"));
        assert!(e.contains("语音使用"));
    }
}
