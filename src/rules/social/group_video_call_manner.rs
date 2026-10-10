//! 群视频通话礼仪
//!
//! 多人视频通话中的发言秩序与设备使用规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GroupVideoCallMannerRules,
    name: "群视频通话礼仪",
    desc: "多人视频通话中的发言秩序与设备使用规范",
    origin: "网络通用",
    tags: ["线上", "会议", "礼仪"]
}

impl GroupVideoCallMannerRules {
    /// 入会准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前测试设备网络",
            "选安静整洁环境",
            "修改为真实姓名",
            "准时入会不迟到",
        ]
    }

    /// 发言秩序
    pub fn speaking(&self) -> Vec<&'static str> {
        vec![
            "非发言时静音",
            "按顺序依次发言",
            "不随意打断他人",
            "用举手功能示意",
        ]
    }

    /// 镜头举止
    pub fn camera(&self) -> Vec<&'static str> {
        vec![
            "按约定开摄像头",
            "保持专注不玩手机",
            "不做夸张表情动作",
            "避免在镜头前走动",
        ]
    }

    /// 互动礼节
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "聊天区发言简洁",
            "不刷屏不闲聊",
            "重要事项打字确认",
            "会后整理待办",
        ]
    }
}

impl Rule for GroupVideoCallMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("group_video_call_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【群视频通话礼仪】\n{}",
            [
                format!(
                    "入会准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发言秩序：\\n{}",
                    self.speaking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "镜头举止：\\n{}",
                    self.camera()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "互动礼节：\\n{}",
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
    fn test_groupvideocallmannerrules_basic() {
        let rules = GroupVideoCallMannerRules::new();
        assert_eq!(rules.metadata().name, "群视频通话礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.speaking().is_empty());
        assert!(!rules.camera().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_groupvideocallmannerrules_validation() {
        let rules = GroupVideoCallMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("group_video_call_manner")
        );
    }

    #[test]
    fn test_groupvideocallmannerrules_explain() {
        let rules = GroupVideoCallMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("入会准备"));
        assert!(e.contains("发言秩序"));
        assert!(e.contains("镜头举止"));
    }
}
