//! 家长会沟通礼仪
//!
//! 参加家长会与老师沟通的礼仪规则，包括守时、尊重、理性沟通与家校配合，共同促进孩子成长。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ParentTeacherMeetingRules,
    name: "家长会沟通礼仪",
    desc: "参加家长会与老师沟通的礼仪规则，包括守时、尊重、理性沟通与家校配合，共同促进孩子成长。",
    origin: "家校共育场景中的常见沟通礼仪",
    tags: ["社交", "教育", "家校", "礼仪"]
}

impl ParentTeacherMeetingRules {
    /// 会前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "按时到场不迟到",
            "提前了解孩子在校情况",
            "梳理想沟通的问题",
            "手机调至静音",
        ]
    }

    /// 沟通态度
    pub fn attitude(&self) -> Vec<&'static str> {
        vec![
            "尊重老师的专业判断",
            "耐心倾听不打断",
            "就事论事不指责",
            "不攀比不抱怨",
        ]
    }

    /// 交流内容
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "客观反映孩子在家表现",
            "重点询问具体改进建议",
            "不只听成绩也关注身心",
            "有疑问平和求证",
        ]
    }

    /// 家校配合
    pub fn cooperation(&self) -> Vec<&'static str> {
        vec![
            "认同老师合理要求",
            "在家配合落实教育方法",
            "保持长期沟通渠道",
            "会后向孩子转达关怀",
        ]
    }
}

impl Rule for ParentTeacherMeetingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("parent_teacher")
    }

    fn explain(&self) -> String {
        format!(
            "【家长会沟通礼仪】\n{}",
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
                    "沟通态度：\\n{}",
                    self.attitude()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交流内容：\\n{}",
                    self.communication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家校配合：\\n{}",
                    self.cooperation()
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
    fn test_parentteachermeetingrules_basic() {
        let rules = ParentTeacherMeetingRules::new();
        assert_eq!(rules.metadata().name, "家长会沟通礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.attitude().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.cooperation().is_empty());
    }

    #[test]
    fn test_parentteachermeetingrules_validation() {
        let rules = ParentTeacherMeetingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("parent_teacher"));
    }

    #[test]
    fn test_parentteachermeetingrules_explain() {
        let rules = ParentTeacherMeetingRules::new();
        let e = rules.explain();
        assert!(e.contains("会前准备"));
        assert!(e.contains("沟通态度"));
        assert!(e.contains("交流内容"));
    }
}
