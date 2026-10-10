//! 远程会议礼仪
//!
//! 线上视频会议中的设备、发言与互动规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RemoteMeetingEtiquetteRules,
    name: "远程会议礼仪",
    desc: "线上视频会议中的设备、发言与互动规范",
    origin: "职场通用",
    tags: ["职场", "线上", "会议"]
}

impl RemoteMeetingEtiquetteRules {
    /// 会前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前测试音视频设备",
            "选安静整洁的背景",
            "备好资料准时入会",
            "修改显示名为真实姓名",
        ]
    }

    /// 发言规范
    pub fn speaking(&self) -> Vec<&'static str> {
        vec![
            "非发言时静音",
            "发言前先示意再开口",
            "一次一人避免抢话",
            "语速平稳吐字清晰",
        ]
    }

    /// 镜头礼仪
    pub fn camera(&self) -> Vec<&'static str> {
        vec![
            "按约定开启摄像头",
            "正视镜头保持专注",
            "避免边吃边开会",
            "不做引人分心的动作",
        ]
    }

    /// 互动秩序
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "用举手或聊天区示意",
            "重要结论打字记录",
            "不私聊刷屏干扰",
            "会后整理行动项",
        ]
    }
}

impl Rule for RemoteMeetingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("remote_meeting_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【远程会议礼仪】\n{}",
            [
                format!(
                    "会前准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发言规范：\\n{}",
                    self.speaking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "镜头礼仪：\\n{}",
                    self.camera()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "互动秩序：\\n{}",
                    self.interaction()
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
    fn test_remotemeetingetiquetterules_basic() {
        let rules = RemoteMeetingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "远程会议礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.speaking().is_empty());
        assert!(!rules.camera().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_remotemeetingetiquetterules_validation() {
        let rules = RemoteMeetingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("remote_meeting_etiquette")
        );
    }

    #[test]
    fn test_remotemeetingetiquetterules_explain() {
        let rules = RemoteMeetingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("会前准备"));
        assert!(e.contains("发言规范"));
        assert!(e.contains("镜头礼仪"));
    }
}
