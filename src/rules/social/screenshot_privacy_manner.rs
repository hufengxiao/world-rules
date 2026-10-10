//! 截图隐私礼仪
//!
//! 聊天截图与转发时的隐私保护与征询规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ScreenshotPrivacyMannerRules,
    name: "截图隐私礼仪",
    desc: "聊天截图与转发时的隐私保护与征询规范",
    origin: "网络通用",
    tags: ["线上", "隐私", "礼仪"]
}

impl ScreenshotPrivacyMannerRules {
    /// 转发征询
    pub fn consent(&self) -> Vec<&'static str> {
        vec![
            "转发他人对话先征得同意",
            "涉及隐私须谨慎",
            "不打码不转发敏感内容",
            "尊重对话的私密性",
        ]
    }

    /// 打码处理
    pub fn masking(&self) -> Vec<&'static str> {
        vec![
            "隐去姓名头像信息",
            "遮挡手机号与地址",
            "模糊可识别特征",
            "避免暴露他人身份",
        ]
    }

    /// 使用边界
    pub fn boundaries(&self) -> Vec<&'static str> {
        vec![
            "不用于恶意攻击他人",
            "不公开私人聊天内容",
            "群内截图勿外传",
            "职场信息注意保密",
        ]
    }

    /// 被截图应对
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "发现被侵权可要求删除",
            "保留证据维权",
            "与对方沟通说明",
            "严重者依法处理",
        ]
    }
}

impl Rule for ScreenshotPrivacyMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("screenshot_privacy_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【截图隐私礼仪】\n{}",
            [
                format!(
                    "转发征询：\\n{}",
                    self.consent()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "打码处理：\\n{}",
                    self.masking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "使用边界：\\n{}",
                    self.boundaries()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "被截图应对：\\n{}",
                    self.response()
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
    fn test_screenshotprivacymannerrules_basic() {
        let rules = ScreenshotPrivacyMannerRules::new();
        assert_eq!(rules.metadata().name, "截图隐私礼仪");
        assert!(!rules.consent().is_empty());
        assert!(!rules.masking().is_empty());
        assert!(!rules.boundaries().is_empty());
        assert!(!rules.response().is_empty());
    }

    #[test]
    fn test_screenshotprivacymannerrules_validation() {
        let rules = ScreenshotPrivacyMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("screenshot_privacy_manner")
        );
    }

    #[test]
    fn test_screenshotprivacymannerrules_explain() {
        let rules = ScreenshotPrivacyMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("转发征询"));
        assert!(e.contains("打码处理"));
        assert!(e.contains("使用边界"));
    }
}
