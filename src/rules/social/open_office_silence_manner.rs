//! 开放办公安静礼仪
//!
//! 开放式办公区保持安静与减少干扰的规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OpenOfficeSilenceMannerRules,
    name: "开放办公安静礼仪",
    desc: "开放式办公区保持安静与减少干扰的规范",
    origin: "职场通用",
    tags: ["职场", "办公", "礼仪"]
}

impl OpenOfficeSilenceMannerRules {
    /// 音量控制
    pub fn volume(&self) -> Vec<&'static str> {
        vec![
            "交谈轻声细语",
            "戴耳机不放大音量",
            "手机调振动静音",
            "避免大声讲电话",
        ]
    }

    /// 通话处理
    pub fn calls(&self) -> Vec<&'static str> {
        vec![
            "长谈移至电话间",
            "视频会议用耳机",
            "免提通话找独立空间",
            "控制通话时长",
        ]
    }

    /// 行为注意
    pub fn behavior(&self) -> Vec<&'static str> {
        vec![
            "键盘敲击轻柔",
            "不吃气味浓食物",
            "走动脚步放轻",
            "不频繁制造声响",
        ]
    }

    /// 相互体谅
    pub fn consideration(&self) -> Vec<&'static str> {
        vec![
            "有事小声沟通",
            "必要交流用即时消息",
            "遇噪音委婉提醒",
            "共同营造专注环境",
        ]
    }
}

impl Rule for OpenOfficeSilenceMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("open_office_silence_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【开放办公安静礼仪】\n{}",
            [
                format!(
                    "音量控制：\\n{}",
                    self.volume()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "通话处理：\\n{}",
                    self.calls()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行为注意：\\n{}",
                    self.behavior()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相互体谅：\\n{}",
                    self.consideration()
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
    fn test_openofficesilencemannerrules_basic() {
        let rules = OpenOfficeSilenceMannerRules::new();
        assert_eq!(rules.metadata().name, "开放办公安静礼仪");
        assert!(!rules.volume().is_empty());
        assert!(!rules.calls().is_empty());
        assert!(!rules.behavior().is_empty());
        assert!(!rules.consideration().is_empty());
    }

    #[test]
    fn test_openofficesilencemannerrules_validation() {
        let rules = OpenOfficeSilenceMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("open_office_silence_manner")
        );
    }

    #[test]
    fn test_openofficesilencemannerrules_explain() {
        let rules = OpenOfficeSilenceMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("音量控制"));
        assert!(e.contains("通话处理"));
        assert!(e.contains("行为注意"));
    }
}
