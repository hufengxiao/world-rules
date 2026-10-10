//! 会议室预订礼仪
//!
//! 共用会议室预订与使用的时间规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MeetingRoomBookingMannerRules,
    name: "会议室预订礼仪",
    desc: "共用会议室预订与使用的时间规范",
    origin: "职场通用",
    tags: ["职场", "共用", "礼仪"]
}

impl MeetingRoomBookingMannerRules {
    /// 预订规范
    pub fn booking(&self) -> Vec<&'static str> {
        vec![
            "按需预订合理时长",
            "如实填写会议信息",
            "勿长期占用",
            "变更及时取消",
        ]
    }

    /// 使用礼节
    pub fn usage(&self) -> Vec<&'static str> {
        vec![
            "准时开始结束",
            "不超时占用",
            "结束后恢复桌椅",
            "带走个人物品",
        ]
    }

    /// 设备维护
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "正确操作投影设备",
            "用完关闭电源",
            "爱护公共设施",
            "损坏及时报修",
        ]
    }

    /// 共享意识
    pub fn awareness(&self) -> Vec<&'static str> {
        vec![
            "让急需者优先使用",
            "小事不必占大间",
            "保持室内整洁",
            "尊重他人预约",
        ]
    }
}

impl Rule for MeetingRoomBookingMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("meeting_room_booking_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【会议室预订礼仪】\n{}",
            [
                format!(
                    "预订规范：\\n{}",
                    self.booking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "使用礼节：\\n{}",
                    self.usage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "设备维护：\\n{}",
                    self.equipment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共享意识：\\n{}",
                    self.awareness()
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
    fn test_meetingroombookingmannerrules_basic() {
        let rules = MeetingRoomBookingMannerRules::new();
        assert_eq!(rules.metadata().name, "会议室预订礼仪");
        assert!(!rules.booking().is_empty());
        assert!(!rules.usage().is_empty());
        assert!(!rules.equipment().is_empty());
        assert!(!rules.awareness().is_empty());
    }

    #[test]
    fn test_meetingroombookingmannerrules_validation() {
        let rules = MeetingRoomBookingMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("meeting_room_booking_manner")
        );
    }

    #[test]
    fn test_meetingroombookingmannerrules_explain() {
        let rules = MeetingRoomBookingMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("预订规范"));
        assert!(e.contains("使用礼节"));
        assert!(e.contains("设备维护"));
    }
}
